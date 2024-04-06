use crate::character::CharacterState;
use crate::environment::TILE_SIZE;
use crate::events::NewPlayerCommand;
use bevy::prelude::{Camera, Commands, Entity, EventReader, Query, Res, Transform, Vec3, With};
use crate::animation_scenes::AnimationScene;
use crate::AppState;

pub fn resolve_player_commands(
    mut commands: Commands,
    mut events: EventReader<NewPlayerCommand>
) {
    for event in events.read() {
        match event {
            NewPlayerCommand::Move { x, z } => {
                let world_x = (*x as f32) * TILE_SIZE;
                let world_z = (*z as f32) * TILE_SIZE;

                commands.insert_resource(AppState::Animating(
                    AnimationScene::PlayerMove { x: world_x, z: world_z})
                );
            }
        }
    }
}
