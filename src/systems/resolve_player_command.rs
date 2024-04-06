use crate::character::CharacterState;
use crate::environment::TILE_SIZE;
use crate::events::NewPlayerCommand;
use bevy::prelude::{
    Camera, Camera3dBundle, Commands, Entity, EventReader, Query, Transform, Vec3, With, Without,
};

pub fn resolve_player_commands(
    mut commands: Commands,
    mut events: EventReader<NewPlayerCommand>,
    mut character_query: Query<&mut Transform, With<CharacterState>>,
    mut camera_query: Query<Entity, With<Camera>>,
) {
    for event in events.read() {
        match event {
            NewPlayerCommand::Move { x, z } => {
                let world_x = (*x as f32) * TILE_SIZE;
                let world_z = (*z as f32) * TILE_SIZE;
                if let (mut transform) = character_query.single_mut() {
                    transform.translation.x = world_x;
                    transform.translation.z = world_z;
                }

                if let (entity) = camera_query.single_mut() {
                    commands.entity(entity).insert(
                        Transform::from_xyz(10.0 + world_x, 10.0, 15.5 + world_z)
                            .looking_at(Vec3::new(world_x, 1.0, world_z), Vec3::Y),
                    );
                }
            }
        }
    }
}
