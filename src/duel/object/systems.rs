use bevy::prelude::*;

use super::components::*;

pub fn apply_accel_to_speed(mut query: Query<(&mut Speed, &Acceleration)>) {
    for (mut speed, acceleration) in &mut query {
        speed.0 += acceleration.0;
    }
}

pub fn apply_speed_to_force(mut query: Query<(&mut ForceAccum, &Speed, &Dir)>) {
    for (mut force, speed, dir) in &mut query {
        force.0 += dir.0.normalize_or_zero() * speed.0;
    }
}

pub fn apply_force_to_next_position(
    mut query: Query<(&mut NextPosition, &Transform, &ForceAccum)>,
    time: Res<Time>,
) {
    for (mut next_position, transform, force) in &mut query {
        let force_xyz = Vec3::new(force.0.x, force.0.y, 0.0);
        next_position.0 += transform.translation + force_xyz * time.delta_seconds();
    }
}

pub fn apply_next_position_to_transform(mut query: Query<(&mut Transform, &NextPosition)>) {
    for (mut transform, position) in &mut query {
       
        transform.translation = position.0;
    }
}

pub fn clear_force_accum(mut query: Query<&mut ForceAccum>) {
    for mut force in &mut query {
        force.0 = Vec2::ZERO;
    }
}

pub fn clear_next_position_accum(mut query: Query<(&mut NextPosition, Without<TagStatic>)>)
{
    for (mut position, _) in &mut query
    {
        position.0 = Vec3::ZERO;
    }
}
