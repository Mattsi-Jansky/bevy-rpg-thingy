use crate::assets::Meshes;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_scene_hook::{HookedSceneBundle, HookPlugin, SceneHook};
use std::f32::consts::PI;
use bevy_mod_raycast::prelude::*;
use crate::map::MAP;

mod assets;
mod environment;
mod map;

fn main() {
    App::new()
        .add_plugins(HookPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultRaycastingPlugin)
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_systems(Startup, (init_meshes, setup).chain())
        .add_systems(Update, setup_scene_once_loaded)
        .add_systems(Update, update_cursor)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

const TILE_SIZE: f32 = 4.;

pub struct CursorRaycast;

fn update_cursor(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_ray) = **cursor_ray {
            let result = raycast.cast_ray(cursor_ray, &default()).get(0);
            if let Some(hit) = result {
                let (_, intersection) = hit;
                let position = intersection.position();
                let x: i32 = (position.x / TILE_SIZE).round() as i32;
                let z: i32 = (position.z / TILE_SIZE).round() as i32;
                info!("Intersection position {}/{}", x, z);
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, meshes: Res<Meshes>) {
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

    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("characters/Rogue.glb#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, commands| {
            if let Some(name) = entity.get::<Name>() {
                if name.contains("Cube.0") {
                    commands.insert(Visibility::Hidden);
                }
            }
        }),
    });

    environment::render_environment(&mut commands, meshes, &MAP);
}

fn init_meshes(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Meshes::init(asset_server));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}
