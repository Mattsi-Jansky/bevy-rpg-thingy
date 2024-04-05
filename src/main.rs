use std::f32::consts::PI;
use bevy::DefaultPlugins;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, HookPlugin, SceneHook};
use rand::prelude::Distribution;
use rand::RngCore;
use lazy_static::lazy_static;
use crate::assets::Meshes;

mod environment;
mod assets;

fn main() {
    App::new()
        .add_plugins(HookPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_meshes, setup).chain())
        .add_systems(
            Update,
            setup_scene_once_loaded,
        )
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

const TILE_SIZE: f32 = 4.;

struct Tile {
    pub tile_type: TileType,
    wall_north: WallType,
    wall_east: WallType,
    wall_south: WallType,
    wall_west: WallType,
}

impl Tile {
    fn new(tile_type: TileType, wall_north: WallType,
           wall_east: WallType,
           wall_south: WallType,
           wall_west: WallType) -> Self {
        Self { tile_type, wall_north, wall_east, wall_south, wall_west }
    }
}

enum TileType {
    None,
    Dirt,
    Tiled,
    Wood
}

enum WallPosition {
    None,
    North,
    East,
    South,
    West,
    NorthEast,
    NorthSouth,
    NorthWest,
    EastSouth,
    EastWest,
    SouthWest
}

enum WallType {
    None,
    Regular
}

lazy_static! {
    static ref map: Vec<Vec<Tile>> = vec![
        vec![
            Tile::new(TileType::Dirt, WallType::Regular, WallType::Regular, WallType::Regular, WallType::Regular),
            Tile::new(TileType::Dirt, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Dirt, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Dirt, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Dirt, WallType::None, WallType::None, WallType::None, WallType::None)
        ],
        vec![
            Tile::new(TileType::Wood, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Wood, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Wood, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Wood, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Wood, WallType::None, WallType::None, WallType::None, WallType::None),
        ],
        vec![
            Tile::new(TileType::Tiled, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Tiled, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::None, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Tiled, WallType::None, WallType::None, WallType::None, WallType::None),
            Tile::new(TileType::Tiled, WallType::None, WallType::None, WallType::None, WallType::None)
        ]
    ];
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Meshes>
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

    commands.spawn(HookedSceneBundle { scene: SceneBundle {
        scene: asset_server.load("characters/Rogue.glb#Scene0"),
        ..default()
    }, hook: SceneHook::new(|entity, commands| {
        if let Some(name) = entity.get::<Name>() {
            if name.contains("Cube.0") {
                commands.insert(Visibility::Hidden);
            }
        }})
    });

    environment::render_environment(&mut commands, meshes, &map);
}

fn init_meshes(mut commands: Commands,
               asset_server: Res<AssetServer>) {
    commands.insert_resource(Meshes::init(asset_server));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}
