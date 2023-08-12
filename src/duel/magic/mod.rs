use bevy::prelude::*;

use self::{events::EventOnWind, systems::on_wind_attack};

pub mod events;
pub mod components;
mod systems;

pub struct MagicPlugin;

impl Plugin for MagicPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_event::<EventOnWind>()
        .add_systems(Update, on_wind_attack);
    }
}