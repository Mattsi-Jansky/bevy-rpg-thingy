use bevy::prelude::{Commands, default, Res, SceneBundle, Transform};
use bevy::asset::AssetServer;
use bevy::math::Quat;
use rand::distributions::{Uniform, Distribution};
use crate::{map, Tile, TILE_SIZE, TileType, WallType};

pub fn render_environment(commands: &mut Commands, asset_server: Res<AssetServer>, map1: &Vec<Vec<Tile>>) {
    let floor_wood = asset_server.load("environment/floor_wood_large.gltf.glb#Scene0");
    let floor_tiled = asset_server.load("environment/floor_tile_large.gltf.glb#Scene0");
    let floor_dirt_a = asset_server.load("environment/floor_dirt_large.gltf.glb#Scene0");
    let floor_dirt_b = asset_server.load("environment/floor_dirt_large_rocky.gltf.glb#Scene0");

    let wall = asset_server.load("environment/wall.gltf.glb#Scene0");
    let wall_corner = asset_server.load("environment/wall_corner.gltf.glb#Scene0");

    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(0, 2);
    let x_size = map.len();
    let z_size = map.get(0).unwrap().len();
    for x in 0..x_size {
        for z in 0..z_size {
            let tile = map.get(x).unwrap().get(z).unwrap();
            let maybe_tile_scene = match tile.tile_type {
                TileType::Dirt => {
                    let random = distribution.sample(&mut rng);
                    Some(
                        if random == 0 { floor_dirt_a.clone_weak() }
                        else { floor_dirt_b.clone_weak() }
                    )
                }
                TileType::Tiled => { Some(floor_tiled.clone_weak()) }
                TileType::Wood => { Some(floor_wood.clone_weak()) }
                TileType::None => { None }
            };
            if let Some(tile_scene) = maybe_tile_scene {
                commands.spawn(SceneBundle {
                    scene: tile_scene,
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)),
                    ..default()
                });
            }
            if matches!(tile.wall_north, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)  - (TILE_SIZE / 2.)),
                    ..default()
                });
            }
            if matches!(tile.wall_west, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) - (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32))
                        .with_rotation(Quat::from_rotation_y(1.5708)),
                    ..default()
                });
            }
            if matches!(tile.wall_east, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) + (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32))
                        .with_rotation(Quat::from_rotation_y(-1.5708)),
                    ..default()
                });
            }
            if matches!(tile.wall_south, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)  + (TILE_SIZE / 2.))
                        .with_rotation(Quat::from_rotation_y(3.14159)),
                    ..default()
                });
            }

            //corners
            if matches!(tile.wall_east, WallType::Regular) && matches!(tile.wall_north, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall_corner.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) + (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32) - (TILE_SIZE / 2.)),
                    ..default()
                });
            }
            if matches!(tile.wall_north, WallType::Regular) && matches!(tile.wall_west, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall_corner.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) - (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32) - (TILE_SIZE / 2.))
                        .with_rotation(Quat::from_rotation_y(1.5708)),
                    ..default()
                });
            }
            if matches!(tile.wall_east, WallType::Regular) && matches!(tile.wall_south, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall_corner.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) + (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32) + (TILE_SIZE / 2.))
                        .with_rotation(Quat::from_rotation_y(-1.5708)),
                    ..default()
                });
            }
            if matches!(tile.wall_south, WallType::Regular) && matches!(tile.wall_west, WallType::Regular) {
                commands.spawn(SceneBundle {
                    scene: wall_corner.clone_weak(),
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32) - (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32) + (TILE_SIZE / 2.))
                        .with_rotation(Quat::from_rotation_y(3.14159)),
                    ..default()
                });
            }
        }
    }
}
