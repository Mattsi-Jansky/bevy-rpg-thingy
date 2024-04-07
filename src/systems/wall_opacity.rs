use bevy::pbr::AlphaMode;
use bevy::prelude::{Assets, Children, Color, Commands, Entity, Handle, HierarchyQueryExt, Query, ResMut, StandardMaterial, With};
use bevy::utils::default;
use crate::world::environment::CameraBlockingWall;

pub fn update_wall_opacity(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_blocking_walls: Query<Entity, With<CameraBlockingWall>>,
    query_material: Query<&Handle<StandardMaterial>>,
    children: Query<&Children>
) {
    for blocking_wall in query_blocking_walls.iter() {
        for child in children.iter_descendants(blocking_wall) {
            if let Ok(_material_handle) = query_material.get(child) {
                commands.entity(child).insert(materials.add(StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    base_color: Color::Rgba{red:1., blue:1., green:1., alpha:0.3},
                    ..default()
                }));
            }
            commands.entity(blocking_wall).remove::<CameraBlockingWall>();
        }
    }
}
