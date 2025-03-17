use super::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerLdtkBundle>("Player")
            .register_type::<PlayerLdtkBundle>();
    }
}
