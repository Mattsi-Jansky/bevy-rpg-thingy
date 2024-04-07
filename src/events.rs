use crate::animation_scenes::AnimationScene;
use crate::world::map_coordinates::MapPoint;
use bevy::prelude::Event;

#[derive(Event)]
pub enum NewPlayerCommand {
    Move { to: MapPoint },
}

#[derive(Event)]
pub struct AnimationSceneStart(pub AnimationScene);
