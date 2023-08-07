use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, RigidBody};

use super::components::*;

pub fn apply_desire_move_to_rigidbody(
    mut query: Query<(&mut ExternalImpulse, &DesireMove, With<RigidBody>)>,
) {
    let drag = 0.9;
    for (mut char, move_desire, _) in &mut query {
        char.impulse = move_desire.0 * drag;
    }
}

pub fn clear_external_impulses(mut query: Query<&mut ExternalImpulse>) {
    for mut imp in &mut query {
        imp.impulse = Vec2::ZERO;
    }
}

pub fn clear_desire_move(mut query: Query<&mut DesireMove>) {
    for mut force in &mut query {
        force.0 = Vec2::ZERO;
    }
}
