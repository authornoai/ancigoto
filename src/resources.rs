use bevy::prelude::*;

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Resource)]
pub struct Wind(pub IVec2);