use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::health_startup;

pub struct HealthPlugin;

impl Plugin for HealthPlugin
{
    fn build(&self, app: &mut App)
    {
        app.
        add_systems(Startup, health_startup);
    }
}