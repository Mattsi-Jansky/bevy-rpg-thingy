use crate::animation_scenes::{AnimationScene, Direction};
use crate::environment::TILE_SIZE;
use crate::events::{AnimationSceneStart, NewPlayerCommand};
use bevy::prelude::{EventReader, EventWriter, Query, Transform, With};
use crate::character::CharacterState;
use crate::world::world_coordinates::WorldPoint;

pub fn resolve_player_commands(
    mut events: EventReader<NewPlayerCommand>,
    mut event_writer: EventWriter<AnimationSceneStart>,
    player_query: Query<&Transform, With<CharacterState>>,
) {
    for event in events.read() {
        let player_position = player_query.single().translation;
        match event {
            NewPlayerCommand::Move { to: to_map } => {
                let from_map = WorldPoint::from(player_position).to_map_point();
                event_writer.send(AnimationSceneStart(AnimationScene::PlayerMove {
                    target: to_map.clone().to_world_point(),
                    direction: Direction::from(&from_map, to_map)
                }));
            }
        }
    }
}
