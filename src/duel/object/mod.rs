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
                clear_external_impulses,
                apply_desire_move_to_rigidbody,
                clear_desire_move,
            )
                .chain(),
        );
    }
}
