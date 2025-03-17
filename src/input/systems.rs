use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{MovementEvent, MovementState};

use super::components::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            .add_systems(Startup, make_input_map)
            .add_systems(Update, handle_input);
    }
}

pub fn make_input_map(mut commands: Commands) {
    let input_map = InputMap::new([
        (InputAction::Jump, KeyCode::Space),
        (InputAction::Jump, KeyCode::KeyM),
        (InputAction::RunEnd, KeyCode::KeyA),
        (InputAction::RunEnd, KeyCode::KeyD),
    ])
    //// TODO: Bring back VirtualAxis, I like it
    .with_axis(InputAction::Run, VirtualAxis::ad());
    commands.spawn(InputManagerBundle::with_map(input_map));
}

pub fn handle_input(
    mut commands: Commands,
    input_query: Query<&ActionState<InputAction>>,
    //mut player_query: Query<&mut MovementState>,
) {
    for input in input_query.iter() {
        if input.just_pressed(&InputAction::Jump) {
            commands.trigger(MovementEvent::Jump);
        }

        //if input.just_pressed(&InputAction::RunLeft) {
        //    commands.trigger(MovementEvent::Run(-1.));
        //}
        //if input.just_pressed(&InputAction::RunRight) {
        //    commands.trigger(MovementEvent::Run(1.));
        //}
        //if input.just_released(&InputAction::RunLeft) {
        //    commands.trigger(MovementEvent::RunEnd);
        //}
        //if input.just_released(&InputAction::RunRight) {
        //    commands.trigger(MovementEvent::RunEnd);
        //}
        let run_amount = input.clamped_value(&InputAction::Run);
        //if run_amount != 0. {
        //    commands.trigger(MovementEvent::Run(run_amount));
        //} else {
        //    //commands.trigger(MovementEvent::RunEnd);
        //    // TODO: Make this work with triggers and just_released
        //    //for mut movement_state in player_query.iter_mut() {
        //    //    *movement_state = MovementState::Idle;
        //    //}
        //}
        //if input.just_released(&InputAction::Run) {
        //}
        if run_amount != 0. {
            commands.trigger(MovementEvent::Run(run_amount));
        }
        if input.just_released(&InputAction::RunEnd) {
            commands.trigger(MovementEvent::RunEnd);
        }
    }
}
