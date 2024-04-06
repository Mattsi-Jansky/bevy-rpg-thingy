use crate::assets::animations::{init_animations};
use crate::assets::meshes::{init_meshes, Meshes};
use crate::camera::setup_camera;
use crate::environment::render_environment;
use crate::events::NewPlayerCommand;
use crate::lighting::setup_lighting;
use crate::map::MAP;
use crate::systems::character_animations::update_character_animations;
use crate::systems::cursor::update_cursor;
use crate::systems::resolve_player_command::resolve_player_commands;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_mod_raycast::prelude::*;
use bevy_scene_hook::HookPlugin;
use character::CharacterBundle;

mod assets;
mod camera;
mod character;
mod environment;
mod events;
mod lighting;
mod map;
mod systems;

fn main() {
    App::new()
        .add_plugins(HookPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultRaycastingPlugin)
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_systems(Startup, (init_meshes, init_animations, setup).chain())
        .add_systems(Update, update_cursor)
        .add_systems(Update, update_character_animations)
        .add_systems(Update, resolve_player_commands)
        .add_event::<NewPlayerCommand>()
        .run();
}

fn setup(mut commands: Commands, meshes: Res<Meshes>) {
    setup_camera(&mut commands);
    setup_lighting(&mut commands);
    commands.spawn(CharacterBundle::new(&meshes));
    render_environment(&mut commands, &meshes, &MAP);
}
