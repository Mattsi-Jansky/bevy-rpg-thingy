use bevy::prelude::{default, MouseButton, Res};
use bevy_mod_raycast::CursorRay;
use bevy_mod_raycast::immediate::Raycast;
use bevy::input::ButtonInput;
use bevy::log::info;
use crate::TILE_SIZE;

pub fn update_cursor(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_ray) = **cursor_ray {
            let result = raycast.cast_ray(cursor_ray, &default()).get(0);
            if let Some(hit) = result {
                let (_, intersection) = hit;
                let position = intersection.position();
                let x: i32 = (position.x / TILE_SIZE).round() as i32;
                let z: i32 = (position.z / TILE_SIZE).round() as i32;
                info!("Intersection position {}/{}", x, z);
            }
        }
    }
}
