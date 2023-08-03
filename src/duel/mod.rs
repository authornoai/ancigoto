use bevy::prelude::*;

//sub-folders
pub mod health;
pub mod object;
pub mod player;
mod gravitation;

//self
mod systems;

use self::gravitation::GravitationPlugin;
use self::health::HealthPlugin;
use self::object::DuelObjectPlugin;
use self::player::PlayerPlugin;
use self::systems::*;

pub struct DuelPlugin;

impl Plugin for DuelPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .add_plugins((
            GravitationPlugin,
            HealthPlugin,
            DuelObjectPlugin,
            PlayerPlugin
        ))
        .add_systems(Startup, spawn_player);
    }
}