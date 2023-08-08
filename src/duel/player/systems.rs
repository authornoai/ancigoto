use bevy::{prelude::*, ecs::query::Has};

use super::components::TagPlayer;
use crate::duel::{fighter::components::MoveVec, object::components::TagGrounded};

pub fn handle_movement_input(
    mut query: Query<(
        &mut MoveVec,
        Has<TagGrounded>,
        With<TagPlayer>,
    )>,
    input: Res<Input<KeyCode>>,
) {
    for (mut move_vec, has_ground, _) in &mut query {
        let mut movement_dir = Vec2::ZERO;

        if input.just_pressed(KeyCode::Space) && has_ground{
            movement_dir.y = 1.0;
        } else if input.pressed(KeyCode::S) {
            //movement_dir.y = -1.0;
        }

        if input.pressed(KeyCode::A) {
            movement_dir.x = -1.0;
        } else if input.pressed(KeyCode::D) {
            movement_dir.x = 1.0;
        }

        move_vec.0 = movement_dir;
    }
}


