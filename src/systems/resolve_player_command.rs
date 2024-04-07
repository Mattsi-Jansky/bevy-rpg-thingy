use crate::animation_scenes::{AnimationScene, Direction};
use crate::world::environment::TILE_SIZE;
use crate::events::{AnimationSceneStart, NewPlayerCommand};
use bevy::prelude::{Commands, EventReader, EventWriter, Query, Transform, With};
use crate::AppState;
use crate::character::CharacterState;
use crate::world::world_coordinates::WorldPoint;

pub fn resolve_player_commands(
    mut events: EventReader<NewPlayerCommand>,
    mut event_writer: EventWriter<AnimationSceneStart>,
    player_query: Query<&Transform, With<CharacterState>>,
    mut commands: Commands
) {
    for event in events.read() {
        let player_position = player_query.single().translation;
        match event {
            NewPlayerCommand::Move { to: to_map } => {
                let from_map = WorldPoint::from(player_position).to_map_point();
                let diff = from_map.diff(to_map);
                if diff == 1 {
                    let scene = AnimationScene::PlayerMove {
                        target: to_map.clone().to_world_point(),
                        direction: Direction::from(&from_map, to_map)
                    };
                    commands.insert_resource(AppState::StartingAnimation);
                    event_writer.send(AnimationSceneStart(scene));
                }
            }
        }
    }
}
