use crate::assets::meshes::Meshes;
use crate::world::map_coordinates::MapPoint;
use bevy::core::Name;
use bevy::prelude::{default, Bundle, Component, Res, SceneBundle, Transform, Visibility};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

impl CharacterBundle {
    pub fn new(meshes: &Res<Meshes>, spawn_point: MapPoint) -> Self {
        let spawn_point_world = spawn_point.to_world_point();
        Self {
            scene: HookedSceneBundle {
                scene: SceneBundle {
                    scene: meshes.character_rogue(),
                    transform: Transform::from_xyz(spawn_point_world.x, 0., spawn_point_world.z),
                    ..default()
                },
                hook: SceneHook::new(|entity, commands| {
                    if let Some(name) = entity.get::<Name>() {
                        if name.contains("Cube.0") {
                            commands.insert(Visibility::Hidden);
                        }
                    }
                }),
            },
            name: Name::new("Player character"),
            state: CharacterState::Idle,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub scene: HookedSceneBundle,
    pub name: Name,
    pub state: CharacterState,
}

#[derive(Component)]
pub enum CharacterState {
    Idle,
    Moving,
}
