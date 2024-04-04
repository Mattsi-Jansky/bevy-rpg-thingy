use bevy::prelude::{Commands, default, Res, SceneBundle, Transform};
use bevy::asset::AssetServer;
use rand::distributions::Uniform;
use crate::{map, TILE_SIZE, TileType};

pub fn render_environment(commands: &mut Commands, asset_server: Res<AssetServer>, map1: &Vec<Vec<TileType>>) {
    let floor_wood = asset_server.load("environment/floor_wood_small.gltf.glb#Scene0");
    let floor_tiled = asset_server.load("environment/floor_tile_small.gltf.glb#Scene0");
    let floor_dirt = asset_server.load("environment/floor_dirt_small_A.gltf.glb#Scene0");
    // let wall = asset_server.load("environment/wall_corner.gltf.glb#Scene0");

    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(0, 3);
    let distribution_2 = Uniform::new(0, 7);
    let x_size = map.len();
    let z_size = map.get(0).unwrap().len();
    for x in 0..x_size {
        for z in 0..z_size {
            let maybe_tile_scene = match map.get(x).unwrap().get(z).unwrap() {
                TileType::Dirt => { Some(floor_dirt.clone_weak()) }
                TileType::Tiled => { Some(floor_tiled.clone_weak()) }
                TileType::Wood => { Some(floor_wood.clone_weak()) }
                TileType::None => { None }
            };
            // let rand_tile = distribution.sample(&mut rng);
            if let Some(tile_scene) = maybe_tile_scene {
                commands.spawn(SceneBundle {
                    scene: tile_scene,
                    transform: Transform::from_xyz(TILE_SIZE * (x as f32), 0., TILE_SIZE * (z as f32)),
                    ..default()
                });
            }
            // if distribution_2.sample(&mut rng) == 0 {
            //     commands.spawn(SceneBundle {
            //         scene: wall.clone_weak(),
            //         transform: Transform::from_xyz(TILE_SIZE * (x as f32) - (TILE_SIZE / 2.), 0., TILE_SIZE * (z as f32)  - (TILE_SIZE / 2.)),
            //         ..default()
            //     });
            // }
        }
    }
}
