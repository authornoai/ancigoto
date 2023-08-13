use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TagMagicBall;

#[derive(Component, Default)]
pub struct TagMagicPart;

#[derive(Component, Default)]
pub struct MagicSequence
{
    pub magic_parts: Vec<u16>
}

