use bevy::prelude::*;

//sub-folders
pub mod health;
pub mod object;
pub mod player;
pub mod fighter;

//self
mod systems;

use self::health::HealthPlugin;
use self::object::DuelObjectPlugin;
use self::player::PlayerPlugin;
use self::fighter::FighterPlugin;

use self::systems::*;

pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HealthPlugin,
            DuelObjectPlugin,
            FighterPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, spawn_player);
    }
}
