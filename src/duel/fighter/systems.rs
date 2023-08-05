use bevy::prelude::*;

use crate::duel::object::components::Acceleration;

use super::components::{MovePower, MoveVec, TagFighter};

pub fn apply_move_desire(
    mut query: Query<(
        &mut MoveVec,
        &mut Acceleration,
        &MovePower,
        With<TagFighter>,
    )>,
) {
    for (mut move_vec, mut acceleration, power, _) in &mut query {
        acceleration.0 += move_vec.0 * power.0;
        move_vec.0 = Vec2::ZERO;
        println!("Accel {}", acceleration.0);
    }
}
