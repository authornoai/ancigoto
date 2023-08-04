use bevy::prelude::*;

#[derive(Event)]
pub struct EvOnCollision
{
    pub target_a : Entity,
    pub target_b : Entity,
    pub target_to_move : Entity,
    pub vector_to_move : Vec3,
}