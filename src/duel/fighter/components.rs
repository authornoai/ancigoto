use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TagFighter;

#[derive(Component, Default)]
pub struct MoveVec(pub Vec2);

#[derive(Component, Default)]
pub struct MovePower(pub Vec2);

#[derive(Bundle)]
pub struct FighterBundle
{
    pub tag: TagFighter,
    pub move_vec: MoveVec,
    pub move_power: MovePower,
}

impl Default for FighterBundle
{
    fn default() -> Self {
        FighterBundle
        {
            move_power: Default::default(),
            move_vec: Default::default(),
            tag: Default::default()
        }
    }
}