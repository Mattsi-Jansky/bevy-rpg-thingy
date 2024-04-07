use crate::environment::TILE_SIZE;
use crate::events::NewPlayerCommand;
use bevy::input::ButtonInput;
use bevy::prelude::{debug, default, EventWriter, MouseButton, Res};
use bevy_mod_raycast::immediate::Raycast;
use bevy_mod_raycast::CursorRay;
use crate::world::map_coordinates::MapPoint;

pub fn update_cursor(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    buttons: Res<ButtonInput<MouseButton>>,
    mut event_writer: EventWriter<NewPlayerCommand>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_ray) = **cursor_ray {
            let result = raycast.cast_ray(cursor_ray, &default()).first();
            if let Some(hit) = result {
                let (_, intersection) = hit;
                let position = intersection.position();
                let x = (position.x / TILE_SIZE).round() as usize;
                let z = (position.z / TILE_SIZE).round() as usize;
                debug!("Clicked on {}/{}", x, z);
                event_writer.send(NewPlayerCommand::Move { to: MapPoint::new(x, z) });
            }
        }
    }
}
