use bevy::prelude::*;

pub mod health;

use crate::duel::health::HealthPlugin;

pub struct DuelPlugin;

impl Plugin for DuelPlugin
{
    fn build(&self, app: &mut App)
    {
        app.
        add_plugins(HealthPlugin);
    }
}