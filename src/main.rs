pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use bevy::prelude::*;

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
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, (spawn_pig, pig_lifetime))
        .run();
}