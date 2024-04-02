use std::f32::consts::PI;
use bevy::DefaultPlugins;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, remove_crossbow).chain())
        .add_systems(
            Update,
            (setup_scene_once_loaded, remove_crossbow),
        )
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Animations(vec![
        asset_server.load("characters/Rogue.glb#Animation2"),
        asset_server.load("characters/Rogue.glb#Animation3"),
        asset_server.load("characters/Rogue.glb#Animation4"),
    ]));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 7.5)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
            .into(),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("characters/Rogue.glb#Scene0"),
        ..default()
    });
}


fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn remove_crossbow(
    player_entities: Query<Entity, Added<AnimationPlayer>>,
    children: Query<&Children>,
    query: Query<(&Name, &Visibility)>,
    mut commands: Commands,
) {
    for player in &player_entities {
        for child in children.iter_descendants(player) {
            for result in query.get(child) {
                if result.0.as_str().contains("Cube.0") {
                    commands.entity(child)
                        .remove::<Visibility>()
                        .insert(Visibility::Hidden);
                }
            }
        }
    }
}
