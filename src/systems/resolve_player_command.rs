use bevy::prelude::{Entity, EventReader, Query, Transform};
use crate::character::CharacterState;
use crate::environment::TILE_SIZE;
use crate::events::NewPlayerCommand;

pub fn resolve_player_commands(
    mut events: EventReader<NewPlayerCommand>,
    mut character_query: Query<(Entity, &CharacterState, &mut Transform)>,
) {
    for event in events.read() {
        match event {
            NewPlayerCommand::Move { x, z } => {
                if let (_, _, mut transform) = character_query.single_mut() {
                    transform.translation.x = (*x as f32) * TILE_SIZE;
                    transform.translation.z = (*z as f32) * TILE_SIZE;
                }
            }
        }
    }
}
