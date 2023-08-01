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

//OLD DATA
#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer
}