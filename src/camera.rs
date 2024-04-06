use bevy::prelude::{Camera3dBundle, Commands, default, Transform};
use bevy::math::Vec3;

pub fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 15.5)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });
}
