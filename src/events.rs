use bevy::prelude::Event;
use crate::animation_scenes::AnimationScene;

#[derive(Event)]
pub enum NewPlayerCommand {
    Move { x: i32, z: i32 },
}

#[derive(Event)]
pub struct AnimationSceneStart(pub AnimationScene);
