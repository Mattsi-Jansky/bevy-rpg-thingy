use crate::environment::TILE_SIZE;
use crate::world::world_coordinates::WorldPoint;

#[derive(Clone, Debug)]
pub struct MapPoint {
    pub x: i32,
    pub z: i32
}

impl MapPoint {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn to_world_point(self) -> WorldPoint {
        WorldPoint {
            x: (self.x as f32) * TILE_SIZE,
            y: 0.0,
            z: (self.z as f32) * TILE_SIZE,
        }
    }
}
