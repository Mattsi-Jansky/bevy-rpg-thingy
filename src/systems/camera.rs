use bevy::math::Vec3;
use bevy::prelude::{Camera, Camera3dBundle, Commands, default, Entity, Query, Transform, With};
use crate::character::CharacterState;

pub fn update_camera(
    mut commands: Commands,
    player_query: Query<&Transform, With<CharacterState>>,
    mut camera_query: Query<Entity, With<Camera>>
) {
    let player_position = player_query.single().translation;
    let camera_entity = camera_query.single_mut();
    commands.entity(camera_entity).insert(
        Transform::from_xyz(-10.0 + player_position.x, 10.0, -15.5 + player_position.z)
            .looking_at(Vec3::new(player_position.x, 1.0, player_position.z), Vec3::Y),
    );
}

pub fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        ..default()
    });
}
