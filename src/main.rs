use crate::animation_scenes::{AnimationScene, AnimationSceneTimer};
use crate::assets::animations::init_animations;
use crate::assets::meshes::{init_meshes, Meshes};
use crate::events::{AnimationSceneStart, NewPlayerCommand};
use crate::lighting::setup_lighting;
use crate::systems::animation_scene_manager::{init_animation_scenes, update_animation_scenes};
use crate::systems::camera::update_camera;
use crate::systems::character_animations::update_character_animations;
use crate::systems::cursor::update_cursor;
use crate::systems::resolve_player_command::resolve_player_commands;
use crate::systems::wall_opacity::update_wall_opacity;
use crate::world::map::generator::generate_map;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_mod_raycast::prelude::*;
use bevy_scene_hook::HookPlugin;
use character::CharacterBundle;
use systems::camera::setup_camera;
use world::environment::render_environment;

mod animation_scenes;
mod assets;
mod bundles;
mod character;
mod events;
mod lighting;
mod systems;
mod world;

#[derive(Resource, Default)]
pub enum AppState {
    #[default]
    AwaitingInput,
    StartingAnimation,
    Animating(AnimationScene),
}

fn main() {
    App::new()
        .add_plugins(HookPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .insert_resource(AppState::default())
        .insert_resource(AnimationSceneTimer::default())
        .add_plugins(DefaultRaycastingPlugin)
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_systems(Startup, (init_meshes, init_animations, setup).chain())
        .add_systems(Update, update_cursor)
        .add_systems(Update, update_character_animations)
        .add_systems(Update, resolve_player_commands)
        .add_systems(Update, update_animation_scenes)
        .add_systems(Update, init_animation_scenes)
        .add_systems(Update, update_camera)
        .add_systems(Update, update_wall_opacity)
        .add_event::<NewPlayerCommand>()
        .add_event::<AnimationSceneStart>()
        .run();
}

fn setup(mut commands: Commands, meshes: Res<Meshes>) {
    setup_camera(&mut commands);
    setup_lighting(&mut commands);
    let map = generate_map();
    render_environment(&mut commands, &meshes, &map.tiles);
    commands.spawn(CharacterBundle::new(&meshes, map.spawn_point.clone()));
    commands.insert_resource(map);
}
