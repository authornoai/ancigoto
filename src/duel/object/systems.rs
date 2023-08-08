use bevy::prelude::*;
use bevy_rapier2d::prelude::{RapierContext, RigidBody, Velocity};

use crate::shared::utils::utils_ground_raycast;

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

pub fn validate_is_grounded(
    query: Query<(Entity, &Transform, &GroundRaycastPos)>,
    context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (entity, transform, raycast_pos) in &query {
        let position = transform.translation.truncate();
        let left_origin = position + raycast_pos.left_pos;
        let right_origin = position + raycast_pos.right_pos;

        let mut test_entity = entity;

        let mut result_toi = Vec2::ZERO;
        if utils_ground_raycast(&context, left_origin, &mut test_entity, &mut result_toi)
            && test_entity != entity
        {
            commands.entity(entity).insert(TagGrounded);
            continue;
        }

        if utils_ground_raycast(&context, right_origin, &mut test_entity, &mut result_toi)
            && test_entity != entity
        {
            commands.entity(entity).insert(TagGrounded);
            continue;
        }

        commands.entity(entity).remove::<TagGrounded>();
    }
}
