use bevy::{color::palettes::css::GREEN, prelude::*};
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_lit::prelude::*;

#[derive(Component, LdtkIntCell)]
#[require(PointLight2d(|| PointLight2d {
    color: Color::from(GREEN),
    intensity: 1.0,
    radius: 200.0,
    falloff: 2.0,
    ..default()
}))]
pub struct LightCrystal {}
