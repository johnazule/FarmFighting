use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_lit::prelude::*;

use crate::GameLayer;

#[derive(Component, Default, Reflect, LdtkIntCell)]
#[require(
    RigidBody(|| RigidBody::Static),
    Collider(|| Collider::rectangle(16., 16.)),
    CollisionLayers(|| CollisionLayers::new(GameLayer::Ground, [
            GameLayer::Default,
            GameLayer::Player,
            GameLayer::Enemy
    ])),
    LightOccluder2d,

)]
// TODO: Change to marker struct if bevy_ecs_ldtk updates
pub struct Platform {}
