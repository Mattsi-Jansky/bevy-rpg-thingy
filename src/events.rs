use bevy::prelude::Event;

#[derive(Event)]
pub enum NewPlayerCommand {
    Move { x: i32, z: i32 },
}
