use bevy::prelude::*;

// Tags
#[derive(Component)]
pub struct TagDuelObject;
#[derive(Component)]
pub struct TagGrounded;

#[derive(Component, Default)]
pub struct GroundRaycastPos {
    pub left_pos: Vec2,
    pub right_pos: Vec2,
}

#[derive(Component, Default)]
pub struct DesireMove(pub Vec2);

#[derive(Bundle)]
pub struct MoveableBundle {
    pub desire_move: DesireMove,
    pub ground_raycast_pos: GroundRaycastPos,
}

impl Default for MoveableBundle {
    fn default() -> Self {
        Self {
            desire_move: Default::default(),
            ground_raycast_pos: Default::default(),
        }
    }
}
