use bevy::asset::Handle;
use bevy::math::Quat;
use bevy::prelude::{Bundle, default, Res, SceneBundle, Transform};
use crate::assets::meshes::Meshes;
use crate::environment::{CameraBlockingWall, TILE_SIZE};
use crate::world::world_coordinates::WorldPoint;

#[derive(Bundle)]
pub struct WallBundleNorth {
    pub scene: SceneBundle
}

#[derive(Bundle)]
pub struct WallBundleEast {
    pub scene: SceneBundle,
    blocking_wall: CameraBlockingWall
}

#[derive(Bundle)]
pub struct WallBundleSouth {
    pub scene: SceneBundle,
    blocking_wall: CameraBlockingWall
}

#[derive(Bundle)]
pub struct WallBundleWest {
    pub scene: SceneBundle
}

impl WallBundleNorth {
    pub fn new(mut world_point: WorldPoint, meshes: &Res<Meshes>) -> Self {
        world_point.z += (TILE_SIZE / 2.);
        Self {
            scene: create_wall_scene(world_point, meshes, 3.1415)
        }
    }
}

impl WallBundleWest {
    pub fn new(mut world_point: WorldPoint, meshes: &Res<Meshes>) -> Self {
        world_point.x += (TILE_SIZE / 2.);
        Self {
            scene: create_wall_scene(world_point, meshes, -1.5708),
        }
    }
}

impl WallBundleSouth {
    pub fn new(mut world_point: WorldPoint, meshes: &Res<Meshes>) -> Self {
        world_point.z -= (TILE_SIZE / 2.);
        Self {
            scene: create_wall_scene(world_point, meshes, 0.),
            blocking_wall: CameraBlockingWall(),
        }
    }
}

impl WallBundleEast {
    pub fn new(mut world_point: WorldPoint, meshes: &Res<Meshes>) -> Self {
        world_point.x -= (TILE_SIZE / 2.);
        Self {
            scene: create_wall_scene(world_point, meshes, 1.5708),
            blocking_wall: CameraBlockingWall(),
        }
    }
}

fn create_wall_scene(world_point: WorldPoint, meshes: &Res<Meshes>, rotation: f32) -> SceneBundle {
    let transform: Transform = world_point.into();
    SceneBundle {
        scene: meshes.wall(),
        transform: transform.with_rotation(Quat::from_rotation_y(rotation)),
        ..default()
    }
}
