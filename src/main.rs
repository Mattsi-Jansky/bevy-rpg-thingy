use crate::assets::meshes::{init_meshes, Meshes};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_scene_hook::HookPlugin;
use bevy_mod_raycast::prelude::*;
use character::CharacterBundle;
use crate::assets::animations::{Animations, init_animations};
use crate::camera::setup_camera;
use crate::environment::render_environment;
use crate::events::NewPlayerCommand;
use crate::lighting::setup_lighting;
use crate::map::MAP;
use crate::systems::character_animations::update_character_animations;
use crate::systems::cursor::update_cursor;

mod environment;
mod map;
mod character;
mod camera;
mod lighting;
mod assets;
mod systems;
mod events;

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
        .add_event::<NewPlayerCommand>()
        .run();
}

fn setup(mut commands: Commands, meshes: Res<Meshes>) {
    setup_camera(&mut commands);
    setup_lighting(&mut commands);
    commands.spawn(CharacterBundle::new(&meshes));
    render_environment(&mut commands, &meshes, &MAP);
}
