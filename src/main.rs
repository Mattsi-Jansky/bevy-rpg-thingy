use crate::assets::Meshes;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_scene_hook::HookPlugin;
use bevy_mod_raycast::prelude::*;
use character::CharacterBundle;
use crate::map::MAP;

mod assets;
mod environment;
mod map;
mod cursor;
mod character;
mod camera;
mod lighting;

fn main() {
    App::new()
        .add_plugins(HookPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultRaycastingPlugin)
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_systems(Startup, (assets::init_meshes, setup).chain())
        .add_systems(Update, setup_scene_once_loaded)
        .add_systems(Update, cursor::update_cursor)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

const TILE_SIZE: f32 = 4.;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, meshes: Res<Meshes>) {
    commands.insert_resource(Animations(vec![
        asset_server.load("characters/Rogue.glb#Animation36"),
        asset_server.load("characters/Rogue.glb#Animation3"),
        asset_server.load("characters/Rogue.glb#Animation4"),
    ]));
    camera::setup_camera(&mut commands);
    lighting::setup_lighting(&mut commands);
    commands.spawn(CharacterBundle::new(&meshes));
    environment::render_environment(&mut commands, &meshes, &MAP);
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}
