use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Res, Resource, Scene};

/// Assumption: Assets will live for whole life of application
#[derive(Resource)]
pub struct Meshes {
    floor_wood: Handle<Scene>,
    floor_tiled: Handle<Scene>,
    floor_dirt_a: Handle<Scene>,
    floor_dirt_b: Handle<Scene>,

    wall: Handle<Scene>,
    wall_corner: Handle<Scene>,
}

impl Meshes {
    pub fn init(asset_server: Res<AssetServer>) -> Self {
        Self {
            floor_wood: asset_server.load("environment/floor_wood_large.gltf.glb#Scene0"),
            floor_tiled: asset_server.load("environment/floor_tile_large.gltf.glb#Scene0"),
            floor_dirt_a: asset_server.load("environment/floor_dirt_large.gltf.glb#Scene0"),
            floor_dirt_b: asset_server.load("environment/floor_dirt_large_rocky.gltf.glb#Scene0"),
            wall: asset_server.load("environment/wall.gltf.glb#Scene0"),
            wall_corner: asset_server.load("environment/wall_corner.gltf.glb#Scene0"),
        }
    }

    pub fn floor_wood(&self) -> Handle<Scene> {
        self.floor_wood.clone_weak()
    }

    pub fn floor_tiled(&self) -> Handle<Scene> {
        self.floor_tiled.clone_weak()
    }

    pub fn floor_dirt_a(&self) -> Handle<Scene> {
        self.floor_dirt_a.clone_weak()
    }

    pub fn floor_dirt_b(&self) -> Handle<Scene> {
        self.floor_dirt_b.clone_weak()
    }

    pub fn wall(&self) -> Handle<Scene> {
        self.wall.clone_weak()
    }

    pub fn wall_corner(&self) -> Handle<Scene> {
        self.wall_corner.clone_weak()
    }
}
