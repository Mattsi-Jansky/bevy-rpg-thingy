use bevy::prelude::{Resource, Timer};
use crate::world::map_coordinates::MapPoint;
use crate::world::world_coordinates::WorldPoint;

#[derive(Clone)]
pub enum AnimationScene {
    PlayerMove { target: WorldPoint, direction: Direction },
}

#[derive(Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn from(from: &MapPoint, to: &MapPoint) -> Direction {
        if from.z < to.z {
            Direction::North
        } else if from.z > to.z {
            Direction::South
        } else if from.x < to.x {
            Direction::West
        } else {
            Direction::East
        }
    }
}

#[derive(Resource, Default)]
pub struct AnimationSceneTimer {
    pub timer: Timer,
}
