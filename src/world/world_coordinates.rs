use bevy::math::Vec3;
use crate::environment::TILE_SIZE;
use crate::world::map_coordinates::MapPoint;

#[derive(Clone, Debug)]
pub struct WorldPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl WorldPoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_map_point(self) -> MapPoint {
        MapPoint {
            x: (self.x / TILE_SIZE).round() as i32,
            z: (self.z / TILE_SIZE).round() as i32,
        }
    }
}

impl From<Vec3> for WorldPoint {
    fn from(point: Vec3) -> Self {
        WorldPoint {
            x: point.x,
            y: point.y,
            z: point.z
        }
    }
}
