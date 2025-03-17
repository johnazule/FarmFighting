use bevy::prelude::*;
use bevy_lit::prelude::Lighting2dPlugin;

use super::components::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Lighting2dPlugin);
    }
}
