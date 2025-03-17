use bevy::prelude::*;
use bevy_lit::prelude::*;

#[derive(Component, Reflect)]
#[require(
    Camera2d,
    Lighting2dSettings,
    AmbientLight2d(|| AmbientLight2d {
        color: Color::WHITE,
        brightness: 0.1
    })
)]
pub struct PlayerCamera;
