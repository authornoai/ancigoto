use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

pub mod systems;

mod duel;
pub mod shared;

use systems::*;

use crate::duel::DuelPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ancigoto".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins(DuelPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(States, Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum AppState
{
    #[default]
    MainMenu,
    Game,
    GameEnd
}