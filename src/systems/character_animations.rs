use crate::assets::animations::Animations;
use crate::character::CharacterState;
use bevy::animation::{AnimationClip, AnimationPlayer};
use bevy::asset::Handle;
use bevy::prelude::{Children, Entity, HierarchyQueryExt, Query, Res};



pub fn update_character_animations(
    character_state_query: Query<(Entity, &CharacterState)>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    children: Query<&Children>,
) {
    for (entity, state) in character_state_query.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok(mut player) = animation_player_query.get_mut(child) {
                let animation = get_animation_for_state(&animations, state);
                if !player.animation_clip().eq(&animation) {
                    player.play(animation).repeat();
                }
            }
        }
    }
}

fn get_animation_for_state(
    animations: &Res<Animations>,
    state: &CharacterState,
) -> Handle<AnimationClip> {
    
    match state {
        CharacterState::Idle => animations.rogue_idle(),
        CharacterState::Moving => animations.rogue_moving()
    }
}
