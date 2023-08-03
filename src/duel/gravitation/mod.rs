use bevy::prelude::*;

mod resources;
mod systems;
pub mod components;

use self::resources::GravityForce;
use self::systems::*;

pub struct GravitationPlugin;

impl Plugin for GravitationPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .insert_resource(GravityForce(Vec2::new(0.0, -9.8)))
        .add_systems(Update, apply_gravity);
    }
}