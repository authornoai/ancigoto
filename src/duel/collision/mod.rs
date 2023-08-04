use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use self::events::*;
use self::systems::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, register_collisions)
        .add_event::<OnCollision>();
    }
}