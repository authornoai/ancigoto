use bevy::prelude::*;

#[derive(Component)]
pub struct Figher
{
    pub is_player : bool
}

#[derive(Component)]
pub struct Direction
{
    pub value: IVec2
}

#[derive(Component)]
pub struct Ball{}
#[derive(Component)]
pub struct Tile{}