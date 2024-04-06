use bevy::prelude::{AnimationClip, AssetServer, Commands, Handle, Res, Resource};

/// Assumption: Animations will live for whole life of application
#[derive(Resource)]
pub struct Animations {
    rogue_idle: Handle<AnimationClip>,
}

impl Animations {
    pub fn init(asset_server: Res<AssetServer>) -> Self {
        Self {
            rogue_idle: asset_server.load("characters/Rogue.glb#Animation36"),
        }
    }

    pub fn rogue_idle(&self) -> Handle<AnimationClip> {
        self.rogue_idle.clone_weak()
    }
}

pub fn init_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Animations::init(asset_server));
}
