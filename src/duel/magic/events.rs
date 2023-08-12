use bevy::prelude::*;

#[derive(Event)]
pub struct EventOnWind
{
    pub pos : Vec2,
    pub dir : Vec2,
    pub power : f32
}