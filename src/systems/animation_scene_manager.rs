use std::time::Duration;
use bevy::math::Vec3;
use bevy::prelude::{Camera, Commands, Entity, EventReader, Query, Res, ResMut, Time, Transform, With};
use bevy::time::{Timer, TimerMode};
use crate::animation_scenes::{AnimationScene, AnimationSceneTimer};
use crate::AppState;
use crate::character::CharacterState;
use crate::environment::TILE_SIZE;
use crate::events::AnimationSceneStart;

pub fn update_animation_scenes(
    app_state: Res<AppState>,
    mut time: Res<Time>,
    mut timer: ResMut<AnimationSceneTimer>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<CharacterState>>,
    mut camera_query: Query<Entity, With<Camera>>
) {
    if let AppState::Animating(animation_scene) = app_state.into_inner() {
        timer.timer.tick(time.delta());
        match animation_scene {
            AnimationScene::PlayerMove { x: target_x, z: target_z } => {
                let (player_entity, mut player_transform) = player_query.single_mut();
                let camera_entity = camera_query.single_mut();

                if !timer.timer.finished() {
                    // player_transform.translation.x += (TILE_SIZE * time.delta_seconds());
                    player_transform.translation.z += (TILE_SIZE * time.delta_seconds());
                    commands.entity(camera_entity).insert(
                        Transform::from_xyz(10.0 + target_x, 10.0, 15.5 + target_z)
                            .looking_at(player_transform.translation, Vec3::Y),
                    );
                }
                else {
                    let (target_x, target_z) = (*target_x, *target_z);
                    player_transform.translation.x = target_x;
                    player_transform.translation.z = target_z;

                    commands.entity(camera_entity).insert(
                        Transform::from_xyz(10.0 + target_x, 10.0, 15.5 + target_z)
                            .looking_at(Vec3::new(target_x, 1.0, target_z), Vec3::Y),
                    );
                    commands.insert_resource(AppState::AwaitingInput);
                    commands.entity(player_entity).insert(CharacterState::Idle);
                }
            }
        }
    }
}

pub fn init_animation_scenes(
    mut events: EventReader<AnimationSceneStart>,
    mut commands: Commands,
    mut player_query: Query<Entity, With<CharacterState>>,
) {
    for event in events.read() {
        let animation_scene = &event.0;
        let player_entity = player_query.single_mut();
        match animation_scene {
            AnimationScene::PlayerMove{x, z} => {
                commands.entity(player_entity).insert(CharacterState::Moving);
                commands.insert_resource(AppState::Animating(
                    AnimationScene::PlayerMove { x: *x, z: *z})
                );
                commands.insert_resource(AnimationSceneTimer { timer: Timer::new(Duration::from_secs(1), TimerMode::Once)});
            }
        }
    }
}
