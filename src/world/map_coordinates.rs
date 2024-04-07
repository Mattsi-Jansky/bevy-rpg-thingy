use crate::world::environment::TILE_SIZE;
use crate::world::world_coordinates::WorldPoint;

#[derive(Clone, Debug)]
pub struct MapPoint {
    pub x: usize,
    pub z: usize,
}

impl MapPoint {
    pub fn new(x: usize, z: usize) -> Self {
        Self { x, z }
    }

    pub fn to_world_point(self) -> WorldPoint {
        WorldPoint {
            x: (self.x as f32) * TILE_SIZE,
            y: 0.0,
            z: (self.z as f32) * TILE_SIZE,
        }
    }

    pub fn diff(&self, other: &MapPoint) -> usize {
        self.x.abs_diff(other.x) + self.z.abs_diff(other.z)
    }
}
