use bevy::prelude::*;

use super::components::*;

pub fn apply_accel_to_speed(mut query: Query<(&mut Speed, &mut Acceleration)>) {
    for (mut speed, mut acceleration) in &mut query {
        speed.0 += acceleration.0;
        speed.0 *= 0.9;
        acceleration.0 = Vec2::ZERO;
    }
}

pub fn apply_speed_to_force(mut query: Query<(&mut ForceAccum, &Speed)>) {
    for (mut force, speed) in &mut query {
        force.0 += speed.0;
    }
}

pub fn clear_force_accum(mut query: Query<&mut ForceAccum>) {
    for mut force in &mut query {
        force.0 = Vec2::ZERO;
    }
}
