use bevy::prelude::*;

macro_rules! import_game_modules {
    ($( $x:ident ),*) => {
        $(
            mod $x {
                pub mod components;
                pub mod systems;
                pub mod prelude {
                    #[allow(unused_imports)]
                    pub use super::components::*;
                    #[allow(unused_imports)]
                    pub use super::systems::*;
                }
            }
            use $x::prelude::*;
        )*
    };
}

import_game_modules!(camera, level, player, movement, input, graphics);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CameraControllerPlugin,
            LevelPlugin,
            PlayerPlugin,
            MovementPlugin,
            InputPlugin,
            GraphicsPlugin,
        ))
        .run();
}
