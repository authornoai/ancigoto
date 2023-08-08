use bevy::prelude::*;

pub mod components;
pub mod systems;

use self::systems::*;

pub struct DuelObjectPlugin;

impl Plugin for DuelObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((
                apply_desire_move_to_rigidbody,
                //validate_is_grounded,
                clear_desire_move,
            )
                .chain(),
            validate_is_grounded),
        );
    }
}
