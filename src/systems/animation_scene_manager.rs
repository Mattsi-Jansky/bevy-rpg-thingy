use crate::animation_scenes::{AnimationScene, AnimationSceneTimer, Direction};
use crate::character::CharacterState;
use crate::world::environment::TILE_SIZE;
use crate::events::AnimationSceneStart;
use crate::AppState;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, EventReader, info, Query, Res, ResMut, Time, Transform, With};
use bevy::time::{Timer, TimerMode};
use std::time::Duration;

pub fn update_animation_scenes(
    app_state: Res<AppState>,
    time: Res<Time>,
    mut timer: ResMut<AnimationSceneTimer>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<CharacterState>>
) {
    if let AppState::Animating(animation_scene) = app_state.into_inner() {
        timer.timer.tick(time.delta());
        match animation_scene {
            AnimationScene::PlayerMove {
                target,
                direction
            } => {
                let (player_entity, mut player_transform) = player_query.single_mut();

                if !timer.timer.finished() {
                    match direction {
                        Direction::North => {player_transform.translation.z += TILE_SIZE * time.delta_seconds()}
                        Direction::East => {player_transform.translation.x -= TILE_SIZE * time.delta_seconds()}
                        Direction::South => {player_transform.translation.z -= TILE_SIZE * time.delta_seconds()}
                        Direction::West => {player_transform.translation.x += TILE_SIZE * time.delta_seconds()}
                    }
                } else {
                    player_transform.translation.x = target.x;
                    player_transform.translation.z = target.z;

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
            AnimationScene::PlayerMove { target, direction } => {
                commands
                    .entity(player_entity)
                    .insert(CharacterState::Moving);
                commands.insert_resource(AppState::Animating(AnimationScene::PlayerMove {
                    target: target.clone(),
                    direction: direction.clone()
                }));
                commands.insert_resource(AnimationSceneTimer {
                    timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
                });
            }
        }
    }
}
