use bevy::math::Vec3;
use bevy::prelude::{Camera, Commands, Entity, Query, Res, Transform, With};
use crate::animation_scenes::AnimationScene;
use crate::AppState;
use crate::character::CharacterState;

pub fn update_animation_scenes(
    app_state: Res<AppState>,
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<CharacterState>>,
    mut camera_query: Query<Entity, With<Camera>>
) {
    if let AppState::Animating(animation_scene) = app_state.into_inner() {
        match animation_scene {
            AnimationScene::PlayerMove { x, z } => {
                let (x, z) = (*x, *z);
                let mut transform = player_query.single_mut();
                transform.translation.x = x;
                transform.translation.z = z;

                let camera_entity = camera_query.single_mut();
                commands.entity(camera_entity).insert(
                    Transform::from_xyz(10.0 + x, 10.0, 15.5 + z)
                        .looking_at(Vec3::new(x, 1.0, z), Vec3::Y),
                );
                commands.insert_resource(AppState::AwaitingInput);
            }
        }
    }
}
