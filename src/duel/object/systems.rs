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

pub fn apply_force_to_position(mut query: Query<(&mut Transform, &ForceAccum)>, time: Res<Time>) {
    for (mut transform, force) in &mut query {
        let force_xyz = Vec3::new(force.0.x, force.0.y, 0.0);
        transform.translation += force_xyz * time.delta_seconds();
    }
}

pub fn clear_force_accum(mut query: Query<&mut ForceAccum>) {
    for mut force in &mut query {
        force.0 = Vec2::ZERO;
    }
}
