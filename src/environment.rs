use crate::assets::meshes::Meshes;
use crate::map::{Tile, TileType, WallType};
use bevy::math::Quat;
use bevy::prelude::{default, Commands, Res, SceneBundle, Transform, Material, StandardMaterial, Handle, info, Component};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use crate::bundles::tile::{IsTile, TileBundle};
use crate::bundles::wall::{WallBundleWest, WallBundleNorth, WallBundleSouth, WallBundleEast};
use crate::world::map_coordinates::MapPoint;

pub const TILE_SIZE: f32 = 4.;

pub fn render_environment(commands: &mut Commands, meshes: &Res<Meshes>, map: &Vec<Vec<Tile>>) {
    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(0, 2);
    let x_size = map.len();
    let z_size = map.first().unwrap().len();
    for x in 0..x_size {
        for z in 0..z_size {
            let map_point = MapPoint::new(x, z);
            let tile = map.get(x).unwrap().get(z).unwrap();
            render_tile(commands, meshes, &mut rng, distribution, x, z, tile);
            render_walls(commands, meshes, map_point, tile);
            render_wall_corners(commands, meshes, x, z, tile);
        }
    }
}

fn render_tile(
    commands: &mut Commands,
    meshes: &Res<Meshes>,
    mut rng: &mut ThreadRng,
    distribution: Uniform<i32>,
    x: usize,
    z: usize,
    tile: &Tile,
) {
    let maybe_tile_scene = match tile.tile_type {
        TileType::Dirt => {
            let random = distribution.sample(&mut rng);
            Some(if random == 0 {
                meshes.floor_dirt_a()
            } else {
                meshes.floor_dirt_b()
            })
        }
        TileType::Tiled => Some(meshes.floor_tiled()),
        TileType::Wood => Some(meshes.floor_wood()),
        TileType::None => None,
    };
    if let Some(tile_scene) = maybe_tile_scene {
        commands.spawn(
            TileBundle {
                scene: SceneBundle {
                    scene: tile_scene,
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)),
                    ..default()
                },
                tile: IsTile(),
            }
        );
    }
}

#[derive(Component)]
pub struct CameraBlockingWall();

fn render_walls(commands: &mut Commands, meshes: &Res<Meshes>, point: MapPoint, tile: &Tile) {
    let world_point = point.to_world_point();
    if matches!(tile.wall_north, WallType::Regular) {
        commands.spawn(WallBundleNorth::new(world_point.clone(), meshes));
    }
    if matches!(tile.wall_east, WallType::Regular) {
        commands.spawn(WallBundleEast::new(world_point.clone(), meshes));
    }
    if matches!(tile.wall_south, WallType::Regular) {
        commands.spawn(WallBundleSouth::new(world_point.clone(), meshes));
    }
    if matches!(tile.wall_west, WallType::Regular) {
        commands.spawn(WallBundleWest::new(world_point.clone(), meshes));
    }
}

fn render_wall_corners(
    commands: &mut Commands,
    meshes: &Res<Meshes>,
    x: usize,
    z: usize,
    tile: &Tile,
) {
    if matches!(tile.wall_west, WallType::Regular) && matches!(tile.wall_south, WallType::Regular) {
        commands.spawn(SceneBundle {
            scene: meshes.wall_corner().clone_weak(),
            transform: Transform::from_xyz(
                TILE_SIZE * (x as f32) + (TILE_SIZE / 2.),
                0.,
                TILE_SIZE * (z as f32) - (TILE_SIZE / 2.),
            ),
            ..default()
        });
    }
    if matches!(tile.wall_south, WallType::Regular) && matches!(tile.wall_east, WallType::Regular) {
        commands.spawn(SceneBundle {
            scene: meshes.wall_corner().clone_weak(),
            transform: Transform::from_xyz(
                TILE_SIZE * (x as f32) - (TILE_SIZE / 2.),
                0.,
                TILE_SIZE * (z as f32) - (TILE_SIZE / 2.),
            )
            .with_rotation(Quat::from_rotation_y(1.5708)),
            ..default()
        });
    }
    if matches!(tile.wall_west, WallType::Regular) && matches!(tile.wall_north, WallType::Regular) {
        commands.spawn(SceneBundle {
            scene: meshes.wall_corner().clone_weak(),
            transform: Transform::from_xyz(
                TILE_SIZE * (x as f32) + (TILE_SIZE / 2.),
                0.,
                TILE_SIZE * (z as f32) + (TILE_SIZE / 2.),
            )
            .with_rotation(Quat::from_rotation_y(-1.5708)),
            ..default()
        });
    }
    if matches!(tile.wall_north, WallType::Regular) && matches!(tile.wall_east, WallType::Regular) {
        commands.spawn(SceneBundle {
            scene: meshes.wall_corner().clone_weak(),
            transform: Transform::from_xyz(
                TILE_SIZE * (x as f32) - (TILE_SIZE / 2.),
                0.,
                TILE_SIZE * (z as f32) + (TILE_SIZE / 2.),
            )
            .with_rotation(Quat::from_rotation_y(3.14159)),
            ..default()
        });
    }
}
