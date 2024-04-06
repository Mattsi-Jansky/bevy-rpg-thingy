use crate::assets::meshes::{init_meshes, Meshes};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_scene_hook::HookPlugin;
use bevy_mod_raycast::prelude::*;
use character::CharacterBundle;
use crate::assets::animations::{Animations, init_animations};
use crate::environment::render_environment;
use crate::map::MAP;

mod environment;
mod map;
mod cursor;
mod character;
mod camera;
mod lighting;
mod assets;

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
        .add_systems(Update, setup_scene_once_loaded)
        .add_systems(Update, cursor::update_cursor)
        .run();
}

fn setup(mut commands: Commands, meshes: Res<Meshes>) {
    camera::setup_camera(&mut commands);
    lighting::setup_lighting(&mut commands);
    commands.spawn(CharacterBundle::new(&meshes));
    render_environment(&mut commands, &meshes, &MAP);
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.rogue_idle()).repeat();
    }
}
