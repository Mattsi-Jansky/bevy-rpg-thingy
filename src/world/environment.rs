use crate::assets::meshes::Meshes;
use crate::bundles::tile::{IsTile, TileBundle};
use crate::bundles::wall::{
    WallBundleEast, WallBundleNorth, WallBundleSouth, WallBundleWest, WallCornerBundleNorthEast,
    WallCornerBundleNorthWest, WallCornerBundleSouthEast, WallCornerBundleSouthWest,
};
use crate::world::map::{Tile, TileType, WallType};
use crate::world::map_coordinates::MapPoint;
use bevy::prelude::{default, Commands, Component, Res, SceneBundle, Transform};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

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
            render_walls(commands, meshes, map_point.clone(), tile);
            render_wall_corners(commands, meshes, map_point, tile);
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
        commands.spawn(TileBundle {
            scene: SceneBundle {
                scene: tile_scene,
                transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)),
                ..default()
            },
            tile: IsTile(),
        });
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
    point: MapPoint,
    tile: &Tile,
) {
    let world_point = point.to_world_point();
    if matches!(tile.wall_west, WallType::Regular) && matches!(tile.wall_south, WallType::Regular) {
        commands.spawn(WallCornerBundleSouthWest::new(world_point.clone(), meshes));
    }
    if matches!(tile.wall_south, WallType::Regular) && matches!(tile.wall_east, WallType::Regular) {
        commands.spawn(WallCornerBundleSouthEast::new(world_point.clone(), meshes));
    }
    if matches!(tile.wall_west, WallType::Regular) && matches!(tile.wall_north, WallType::Regular) {
        commands.spawn(WallCornerBundleNorthWest::new(world_point.clone(), meshes));
    }
    if matches!(tile.wall_north, WallType::Regular) && matches!(tile.wall_east, WallType::Regular) {
        commands.spawn(WallCornerBundleNorthEast::new(world_point.clone(), meshes));
    }
}
