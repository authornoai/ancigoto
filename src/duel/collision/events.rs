use bevy::prelude::*;

#[derive(Event)]
pub struct OnCollision
{
    pub target_a : Entity,
    pub target_b : Entity
}