use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

pub mod components;
pub mod resources;
pub mod systems;

mod duel;

use resources::*;
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
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, (spawn_pig, pig_lifetime))
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