use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum InputAction {
    Jump,
    #[actionlike(Axis)]
    Run,
    //RunRight,
    //RunLeft,
    RunEnd,
}
