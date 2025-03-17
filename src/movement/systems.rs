use avian2d::prelude::*;
use bevy::prelude::*;

use super::components::*;
use crate::Player;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .register_type::<JumpFallState>()
            .add_systems(
                FixedPostUpdate,
                (
                    jump_fall.after(PhysicsSet::StepSimulation),
                    ground_detection,
                    run,
                    //debug_jump_fall.after(PhysicsSet::StepSimulation),
                    debug_jump_fall,
                )
                    .chain(),
            )
            .insert_resource(Gravity::default())
            //.insert_resource(Time::<Fixed>::from_seconds(0.5))
            //.insert_resource(SubstepCount(8))
            .add_observer(movement_validation);
    }
}

/// Decides if `MovementEvent` is valid and if so changes relevent `JumpFallState` and
/// `MovementState`
pub fn movement_validation(
    trigger: Trigger<MovementEvent>,
    mut player_query: Query<
        (
            &mut MovementState,
            &mut JumpFallState,
            &mut JumpCounter,
            &MaxJumpCount,
        ),
        With<Player>,
    >,
) {
    for (mut movement_state, mut jump_fall_state, mut jump_counter, max_jump_count) in
        player_query.iter_mut()
    {
        match trigger.event() {
            MovementEvent::Jump => {
                let can_jump: bool;
                match *jump_fall_state {
                    JumpFallState::JumpStart | JumpFallState::Jumping | JumpFallState::Falling => {
                        can_jump = jump_counter.0 < max_jump_count.0
                    }
                    JumpFallState::Grounded | JumpFallState::JustGrounded => can_jump = true,
                }
                if can_jump {
                    jump_counter.0 += 1;
                    *jump_fall_state = JumpFallState::JumpStart;
                }
            }
            MovementEvent::Run(amount) => {
                if *movement_state == MovementState::Idle {
                    *movement_state = MovementState::RunStart(*amount);
                } else {
                    *movement_state = MovementState::Run(*amount)
                }
            }
            MovementEvent::RunEnd => {
                *movement_state = MovementState::RunEnd;
            }
        }
    }
}

pub fn jump_fall(
    mut jump_fall_query: Query<(
        &mut JumpFallState,
        &JumpHeight,
        &mut JumpCurve,
        &mut JumpCounter,
        //&InitialFallVelocity,
        &MaxFallVelocity,
        &mut FallCurve,
        &mut LinearVelocity,
        &Position,
    )>,
    time: Res<Time>,
) {
    for (
        mut jump_fall_state,
        jump_height,
        mut jump_curve,
        mut jump_counter,
        //initial_fall_velocity,
        max_fall_velocity,
        mut fall_curve,
        mut linear_velocity,
        position,
    ) in jump_fall_query.iter_mut()
    {
        match *jump_fall_state {
            JumpFallState::JumpStart => {
                //info!("Here");
                jump_curve.reset();
                *jump_fall_state = JumpFallState::Jumping;
            }
            JumpFallState::Jumping => {
                let t_delta = time.delta_secs();
                let t_total = jump_curve.timer.duration().as_secs_f32();
                let ticks_in_duration = (t_total / t_delta).floor();
                let percent_of_tick_left = t_total % t_delta;
                let fraction_of_remaining_tick = percent_of_tick_left / ticks_in_duration;
                let t = jump_curve.timer.elapsed_secs();
                let s_derived = (jump_height.0 * t_total) / (t_total + t_delta);
                let a_constant = (-2. * s_derived) / t_total.powi(2);
                let a_area = t_total * a_constant;
                let a = a_constant;
                let u = (2. * s_derived) / t_total;
                let v_prev = linear_velocity.y + a * t_delta;
                let v = u + a * t;
                let v_next = v + a * t_delta;
                let s_calc = (v * t) - (0.5 * a * (t).powi(2));

                linear_velocity.y = v;
                if jump_curve.finished() {
                    *jump_fall_state = JumpFallState::Falling;
                }
                jump_curve.tick(time.delta());
            }
            JumpFallState::Falling => {
                fall_curve.tick(time.delta());
                let new_velocity = max_fall_velocity.0 * fall_curve.current_value();
                linear_velocity.y = linear_velocity.y.lerp(new_velocity, 0.1);
            }
            JumpFallState::JustGrounded => {
                fall_curve.reset();
                jump_counter.0 = 0;
                *jump_fall_state = JumpFallState::Grounded;
            }
            JumpFallState::Grounded => (),
        }
    }
}

pub fn ground_detection(mut query: Query<(&ShapeHits, &mut JumpFallState)>) {
    for (shape_hits, mut jump_fall_state) in query.iter_mut() {
        let is_grounded = !shape_hits.is_empty();
        let is_just_grounded = is_grounded && (*jump_fall_state == JumpFallState::Falling);
        if is_just_grounded {
            //info!("JumpFallState is JustGrounded");
            *jump_fall_state = JumpFallState::JustGrounded;
        }
        if !is_grounded
            && *jump_fall_state != JumpFallState::Jumping
            && *jump_fall_state != JumpFallState::Falling
        {
            *jump_fall_state = JumpFallState::Falling;
        }
    }
}

pub fn run(
    mut query: Query<(
        &mut LinearVelocity,
        &mut RunCurve,
        &MaxRunSpeed,
        &mut MovementState,
    )>,
    time: Res<Time>,
) {
    for (mut linear_velocity, mut run_curve, max_run_speed, mut movement_state) in query.iter_mut()
    {
        match *movement_state {
            MovementState::RunStart(amount) => {
                run_curve.reset();
                linear_velocity.x = 0.;
                *movement_state = MovementState::Run(amount);
            }
            MovementState::Idle => (),
            MovementState::Run(amount) => {
                run_curve.tick(time.delta());
                linear_velocity.x = max_run_speed.0 * run_curve.current_value() * amount;
            }
            MovementState::RunEnd => {
                linear_velocity.x = 0.;
                *movement_state = MovementState::Idle;
            }
        }
    }
}

pub fn debug_jump_fall(query: Query<(&JumpFallState, &LinearVelocity, &Position)>) {
    query
        .iter()
        .for_each(|(jump_fall_state, linear_velocity, position)| {
            //info!("s: {:?}", position.y - 56.);
            //info!("v: {:?}", linear_velocity.y);
            //info!("\n");
            //info!(
            //    "JumpFallState: {:?}\tPosition: {:?}",
            //    jump_fall_state, position
            //)
            // if *jump_fall_state == JumpFallState::Falling {
            //     info!("s: {:?}", position.y - 36.);
            // }
        });
}
