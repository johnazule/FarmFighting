use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    movement::prelude::{FallCurve, MovementCurve},
    Jumper, MaxFallVelocity, MaxJumpCount, Mover,
};

// TODO: change from Bundle to marker component when bevy_ecs_ldtk implements this
#[derive(Bundle, LdtkEntity, Default, Reflect)]
pub struct PlayerLdtkBundle {
    #[sprite_sheet("sprites/hehe.png", 10, 20, 6, 3, 1, 0, 0)]
    sprite: Sprite,
    #[worldly]
    worldly: Worldly,
    player: Player,
}

#[derive(Default, Component, Reflect)]
#[require(
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::rectangle(10., 20.)),
    Mover,
    MaxJumpCount(|| MaxJumpCount(3)),
    FallCurve(|| FallCurve(MovementCurve::default().with_duration_secs(0.2))),
    MaxFallVelocity(|| MaxFallVelocity(-600.))
)]
pub struct Player;
