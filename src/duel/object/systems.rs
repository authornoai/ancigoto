use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBody, Velocity};

use super::components::*;

pub fn apply_desire_move_to_rigidbody(
    mut query: Query<(&mut Velocity, &DesireMove, With<RigidBody>)>,
) {
    let drag = 0.98;
    for (mut char, move_desire, _) in &mut query {
        char.linvel += move_desire.0;
        char.linvel.x *= drag;
    }
}

pub fn clear_desire_move(mut query: Query<&mut DesireMove>) {
    for mut move_desire in &mut query {
        move_desire.0 = Vec2::ZERO;
    }
}
