use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::{RapierContext, RigidBody, Velocity};

use crate::shared::utils::utils_ground_raycast;

use super::components::*;

pub fn apply_desire_move_to_rigidbody(
    mut query: Query<(&mut Velocity, &mut DesireMove, With<RigidBody>)>,
) {
    let drag = 0.98;
    for (mut char, mut move_desire, _) in &mut query {
        char.linvel += move_desire.0;
        char.linvel.x *= drag;
        move_desire.0 = Vec2::ZERO;
    }
}

pub fn validate_is_grounded(
    query: Query<(Entity, &Transform, &GroundRaycastPos)>,
    context: Res<RapierContext>,
    mut lines: ResMut<DebugLines>,
    mut commands: Commands,
) {
    for (entity, transform, raycast_pos) in &query {
        let position = transform.translation.truncate();
        let left_origin = position + raycast_pos.left_pos;
        let right_origin = position + raycast_pos.right_pos;

        let mut test_entity = entity;
        println!("{:?}", test_entity);

        let mut result_toi = Vec2::ZERO;
        lines.line(left_origin.extend(0.0), (left_origin + Vec2::NEG_Y * raycast_pos.max_toi).extend(0.0), 0.0);
        lines.line(right_origin.extend(0.0), (right_origin + Vec2::NEG_Y * raycast_pos.max_toi).extend(0.0), 0.0);
        if utils_ground_raycast(&context, left_origin, raycast_pos.max_toi,&mut test_entity, &mut result_toi)
            && test_entity != entity
        {
            commands.entity(entity).insert(TagGrounded);
            continue;
        }

        println!("{:?}", test_entity);

        if utils_ground_raycast(&context, right_origin, raycast_pos.max_toi, &mut test_entity, &mut result_toi)
            && test_entity != entity
        {
            commands.entity(entity).insert(TagGrounded);
            continue;
        }

        println!("{:?}", test_entity);

        commands.entity(entity).remove::<TagGrounded>();
    }
}
