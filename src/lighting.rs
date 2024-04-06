use bevy::prelude::{Commands, default, Transform};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::math::{EulerRot, Quat};
use std::f32::consts::PI;

pub fn setup_lighting(commands: &mut Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
            .into(),
        ..default()
    });
}
