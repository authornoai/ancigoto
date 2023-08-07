use bevy::prelude::*;

use crate::duel::object::components::DesireMove;

use super::components::{MovePower, MoveVec, TagFighter};

pub fn apply_move_desire(
    mut query: Query<(
        &mut MoveVec,
        &mut DesireMove,
        &MovePower,
        With<TagFighter>,
    )>,
) {
    for (mut move_vec, mut move_desire, power, _) in &mut query {
        move_desire.0 += move_vec.0 * power.0;
        move_vec.0 = Vec2::ZERO;
    }
}
