use bevy::prelude::*;

pub mod components;
pub mod systems;

use self::systems::*;

pub struct DuelObjectPlugin;

impl Plugin for DuelObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_next_position_to_transform,
                clear_next_position_accum,
                apply_accel_to_speed,
                apply_speed_to_force,
                apply_force_to_next_position,
                clear_force_accum,
            )
                .chain(),
        );
    }
}
