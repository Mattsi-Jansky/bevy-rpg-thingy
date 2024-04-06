use crate::animation_scenes::AnimationScene;
use bevy::prelude::Event;

#[derive(Event)]
pub enum NewPlayerCommand {
    Move { x: i32, z: i32 },
}

#[derive(Event)]
pub struct AnimationSceneStart(pub AnimationScene);
