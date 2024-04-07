use crate::animation_scenes::AnimationScene;
use bevy::prelude::Event;
use crate::world::map_coordinates::MapPoint;

#[derive(Event)]
pub enum NewPlayerCommand {
    Move{to: MapPoint},
}

#[derive(Event)]
pub struct AnimationSceneStart(pub AnimationScene);
