use bevy::prelude::*;

use super::components::TagGravity;
use super::resources::GravityForce;
use crate::duel::object::components::ForceAccum;

pub fn apply_gravity(
    mut query: Query<(&mut ForceAccum, With<TagGravity>)>,
    gravity_force: Res<GravityForce>,
)
{
    let gravity_vector = gravity_force.0; 

    for (mut force_acum, _) in &mut query
    {
        force_acum.0 += gravity_vector;
    }
}