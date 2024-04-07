use crate::environment::TILE_SIZE;
use crate::events::NewPlayerCommand;
use bevy::input::ButtonInput;
use bevy::prelude::{debug, default, EventWriter, HierarchyQueryExt, MouseButton, Parent, Query, Res};
use bevy_mod_raycast::immediate::Raycast;
use bevy_mod_raycast::CursorRay;
use bevy_mod_raycast::prelude::RaycastSettings;
use crate::bundles::tile::IsTile;
use crate::world::map_coordinates::MapPoint;

pub fn update_cursor(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    buttons: Res<ButtonInput<MouseButton>>,
    mut event_writer: EventWriter<NewPlayerCommand>,
    tile_query: Query<&IsTile>,
    parent_query: Query<&Parent>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_ray) = **cursor_ray {
            let filter = |entity| {
                for parent in parent_query.iter_ancestors(entity) {
                    if tile_query.contains(parent) { return true; }
                }
                return false;
            };
            let result = raycast.cast_ray(cursor_ray, &RaycastSettings::default().with_filter(&filter)).first();
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
