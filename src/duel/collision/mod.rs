use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use self::events::*;
use self::systems::*;

use super::object::systems::apply_force_to_next_position;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                register_collisions,
                apply_collision_move_to_force,
                add_collision_move,
            ).after(apply_force_to_next_position),
        )
        .add_event::<EvOnCollision>();
    }
}
