use bevy::prelude::*;

use crate::Player;

use super::components::*;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_camera)
            .add_systems(Update, follow_player)
            .insert_resource(ClearColor(Color::linear_rgb(0.6, 0.3, 0.8)));
    }
}

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((PlayerCamera, Msaa::Off));
}
fn follow_player(
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        for player_transform in player_query.iter() {
            camera_transform.translation = player_transform
                .translation
                .lerp(camera_transform.translation, 0.3);
        }
    }
}
