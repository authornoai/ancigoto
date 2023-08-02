use bevy::prelude::*;

// Tags
#[derive(Component)]
pub struct TagDuelObject;

// Movement
#[derive(Component, Default)]
pub struct Dir(pub Vec2);
#[derive(Component, Default)]
pub struct Speed(pub f32);
#[derive(Component, Default)]
pub struct Acceleration(pub f32);

#[derive(Bundle)]
pub struct MoveableBundle {
    pub dir: Dir,
    pub speed: Speed,
    pub acceleration: Acceleration,
}

impl Default for MoveableBundle {
    fn default() -> Self {
        Self {
            dir: Default::default(),
            speed: Default::default(),
            acceleration: Default::default(),
        }
    }
}
