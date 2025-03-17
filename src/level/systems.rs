use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::graphics::prelude::LightCrystal;

use super::components::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .register_ldtk_int_cell_for_layer::<Platform>("TileIntGrid", 1)
            .register_ldtk_int_cell_for_layer::<LightCrystal>("IntGridLighting", 1)
            .add_systems(Startup, spawn_level);
    }
}

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/level0.ldtk").into(),
        ..default()
    });
}

pub fn spawn_platforms(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    int_cell_query: Query<(Entity, &LdtkIntCell), Added<LdtkIntCell>>,
) {
    int_cell_query.iter().for_each(|(entity, ldtk_int_cell)| {
        if let Some(int_cell_entity) = commands.get_entity(entity) {
            int_cell_entity.insert((Platform, Mesh2d(meshes.add(Rectangle::new(16., 16.)))))
        }
    });
}
