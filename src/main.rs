use std::f32::consts::PI;
use bevy::DefaultPlugins;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::RngCore;

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

const TILE_SIZE: f32 = 2.;

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
        transform: Transform::from_xyz(10.0, 10.0, 15.5)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
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

    let floor_wood = asset_server.load("environment/floor_wood_small.gltf.glb#Scene0");
    let floor_tile = asset_server.load("environment/floor_tile_small.gltf.glb#Scene0");
    let floor_dirt = asset_server.load("environment/floor_dirt_small_A.gltf.glb#Scene0");
    let wall = asset_server.load("environment/wall_corner.gltf.glb#Scene0");

    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(0,3);
    let distribution_2 = Uniform::new(0,7);
    for x in -10..10 {
        for z in -10..10 {
            let rand_tile = distribution.sample(&mut rng);
            commands.spawn(SceneBundle {
                scene: if rand_tile == 0 { floor_wood.clone_weak() } else if rand_tile == 1 { floor_tile.clone_weak() } else { floor_dirt.clone_weak() },
                transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)),
                ..default()
            });
            if distribution_2.sample(&mut rng) == 0 {
                commands.spawn(SceneBundle {
                    scene: wall.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) - (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32)  - (TILE_SIZE / 2.)),
                    ..default()
                });
            }
        }
    }
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
