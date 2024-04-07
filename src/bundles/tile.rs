use bevy::prelude::{Bundle, Component, SceneBundle};

#[derive(Bundle)]
pub struct TileBundle {
    pub scene: SceneBundle,
    pub tile: IsTile
}

#[derive(Component)]
pub struct IsTile();