use bevy::prelude::{Resource, Timer};

pub enum AnimationScene {
    PlayerMove { x: f32, z: f32 },
}

#[derive(Resource, Default)]
pub struct AnimationSceneTimer {
    pub timer: Timer,
}
