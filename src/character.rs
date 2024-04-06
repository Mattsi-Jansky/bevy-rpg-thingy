use bevy::prelude::{Bundle, default, Res, SceneBundle, Visibility};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use bevy::core::Name;
use crate::assets::Meshes;

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
            name: Name::new("Player character")
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub scene: HookedSceneBundle,
    pub name: Name
}
