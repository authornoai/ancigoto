use bevy::prelude::*;

// Tags
#[derive(Component)]
pub struct TagDuelObject;

#[derive(Component, Default)]
pub struct DesireMove(pub Vec2);

#[derive(Bundle)]
pub struct MoveableBundle {
    pub desire_move: DesireMove,
}

impl Default for MoveableBundle {
    fn default() -> Self {
        Self {
            desire_move: Default::default(),
        }
    }
}
