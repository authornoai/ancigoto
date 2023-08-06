use bevy::prelude::*;

use super::components::TagPlayer;
use crate::duel::fighter::components::MoveVec;

pub fn handle_movement_input(
    mut player: Query<(&mut MoveVec, With<TagPlayer>)>,
    input: Res<Input<KeyCode>>,
) {
    let (mut move_vec, _) = player.single_mut();
    let mut movement_dir = Vec2::ZERO;

    if input.pressed(KeyCode::W){
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
