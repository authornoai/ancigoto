use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use self::systems::*;

use super::object::systems::apply_force_to_next_position;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (remove_grounded, handle_collisions).after(apply_force_to_next_position),
        );
    }
}
