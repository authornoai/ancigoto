use bevy::prelude::*;
use bevy_rapier2d::prelude::{RapierContext, RigidBody, Velocity};

use crate::{shared::utils::utils_ground_raycast, duel::magic::components::MagicWind};

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
    mut commands: Commands,
) {
    for (entity, transform, raycast_pos) in &query {
        let position = transform.translation.truncate();
        let left_origin = position + raycast_pos.left_pos;
        let right_origin = position + raycast_pos.right_pos;

        let mut test_entity = entity;
        let mut result_toi = Vec2::ZERO;
        if utils_ground_raycast(
            &context,
            left_origin,
            raycast_pos.max_toi,
            &mut test_entity,
            &mut result_toi,
        ) && test_entity != entity
        {
            commands.entity(entity).insert(TagGrounded);
            continue;
        }

        if utils_ground_raycast(
            &context,
            right_origin,
            raycast_pos.max_toi,
            &mut test_entity,
            &mut result_toi,
        ) && test_entity != entity
        {
            commands.entity(entity).insert(TagGrounded);
            continue;
        }

        commands.entity(entity).remove::<TagGrounded>();
    }
}

pub fn try_to_static_magic(
    mut query: Query<(Entity, &Velocity, With<RigidBody>, With<TagMagicRigid>, With<TagGrounded>)>,
    mut commands: Commands,
) {
    for (e, vel, _, _, _) in &mut query {
        
        if vel.linvel.length_squared() > 0.05 {
            continue;
        }

        commands.entity(e).insert(RigidBody::Fixed);
    }
}

pub fn set_impulse_on_wind_attack(
    mut query: Query<(Entity, &mut Velocity, &MagicWind)>,
    mut commands: Commands
) {
    for (e, mut vel, wind) in &mut query
    {
        let wind_dir = wind.0;

        commands.entity(e).remove::<MagicWind>()
        .insert(RigidBody::Dynamic);

        vel.linvel += wind_dir;

        
    }
}
