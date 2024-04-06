use crate::assets::meshes::Meshes;
use bevy::core::Name;
use bevy::prelude::{default, Bundle, Component, Res, SceneBundle, Visibility};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

impl CharacterBundle {
    pub fn new(meshes: &Res<Meshes>) -> Self {
        Self {
            scene: HookedSceneBundle {
                scene: SceneBundle {
                    scene: meshes.character_rogue(),
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
}
