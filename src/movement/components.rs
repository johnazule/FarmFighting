use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub enum MovementEvent {
    Jump,
    Run(f32),
    RunEnd,
}

#[derive(Component, Default, Debug, PartialEq, Reflect)]
pub enum JumpFallState {
    JustGrounded,
    Grounded,
    JumpStart,
    Jumping,
    #[default]
    Falling,
}

#[derive(Component, Default, Debug, PartialEq)]
pub enum MovementState {
    #[default]
    Idle,
    RunStart(f32),
    Run(f32),
    RunEnd,
}

#[derive(Component, Default)]
#[require(
    RigidBody(|| RigidBody::Kinematic),
    Collider,
    Jumper,
    Faller,
    Runner,
    LinearDamping(|| LinearDamping(0.)),
    LockedAxes(|| LockedAxes::ROTATION_LOCKED),
    TransformInterpolation)
]
pub struct Mover;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default, // The name doesn't matter, but Default is used here for clarity.
    Player,
    Enemy,
    Ground,
}

// TODO: Classic, find a better name aha
#[derive(Reflect, Clone, Debug)]
pub struct MovementCurve {
    pub timer: Timer,
    pub curve: CubicSegment<Vec2>,
}

impl Default for MovementCurve {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
            curve: CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)),
        }
    }
}

impl MovementCurve {
    pub fn with_duration_secs(mut self, secs: f32) -> Self {
        self.timer.set_duration(Duration::from_secs_f32(secs));
        self
    }
    pub fn with_curve(mut self, p1: impl Into<Vec2>, p2: impl Into<Vec2>) -> Self {
        self.curve = CubicSegment::new_bezier(p1, p2);
        self
    }
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }
    pub fn current_value(&self) -> f32 {
        self.curve.ease(self.timer.fraction())
    }
    pub fn current_reverse_value(&self) -> f32 {
        self.curve.ease(self.timer.fraction_remaining())
    }
    pub fn current_2nd_derivative_value(&self) -> Vec2 {
        self.curve.acceleration(self.timer.fraction())
    }
    pub fn current_derivative_value(&self) -> Vec2 {
        self.curve.velocity(self.timer.fraction())
    }
    pub fn current_reverse_derivative_value(&self) -> Vec2 {
        -1. * self.curve.velocity(self.timer.fraction_remaining())
    }
    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct JumpHeight(pub f32);

impl Default for JumpHeight {
    fn default() -> Self {
        Self(100.)
    }
}

#[derive(Component, Reflect, DerefMut, Deref)]
pub struct JumpImpulse(pub f32);

impl Default for JumpImpulse {
    fn default() -> Self {
        Self(100.)
    }
}

#[derive(Component, Reflect, Default, Deref, DerefMut, Debug, Clone)]
pub struct JumpCurve(pub MovementCurve);

impl Into<MovementCurve> for JumpCurve {
    fn into(self) -> MovementCurve {
        self.0
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct MaxJumpCount(pub i32);

impl Default for MaxJumpCount {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component, Default, Reflect, Deref, DerefMut)]
// TODO: Decide wheter this require goes here or in `Jumper`
#[require(MaxJumpCount)]
pub struct JumpCounter(pub i32);

#[derive(Component, Reflect, Default)]
#[require(
    JumpHeight(|| JumpHeight(100.)),
    JumpImpulse,
    JumpCurve(|| JumpCurve(
            MovementCurve::default()
                .with_duration_secs(0.5)
                .with_curve((0.,0.,), (1.,1.))
            )),
    JumpFallState,
    JumpCounter,
    ShapeCaster(|| ShapeCaster::new(
            Collider::rectangle(8., 20.),
            Vec2::ZERO,
            0.,
            Dir2::NEG_Y
        ).with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Ground))
        .with_max_distance(10.)
    ),
)]
pub struct Jumper;

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct InitialFallVelocity(pub f32);

impl Default for InitialFallVelocity {
    fn default() -> Self {
        Self(-200.)
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct MaxFallVelocity(pub f32);

impl Default for MaxFallVelocity {
    fn default() -> Self {
        Self(-300.)
    }
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
pub struct FallCurve(pub MovementCurve);

#[derive(Component, Reflect, Default)]
#[require(
    InitialFallVelocity,
    MaxFallVelocity,
    FallCurve,
    JumpFallState,
    GravityScale(|| GravityScale(0.))
)]
pub struct Faller;

#[derive(Component, Reflect, Deref, DerefMut, Debug, Default)]
pub struct RunCurve(pub MovementCurve);

#[derive(Component, Reflect, Deref, DerefMut, Debug)]
pub struct MaxRunSpeed(pub f32);

impl Default for MaxRunSpeed {
    fn default() -> Self {
        Self(100.)
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[require(RunCurve, MaxRunSpeed, MovementState)]
pub struct Runner;
