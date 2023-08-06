use bevy::prelude::*;

// Tags
#[derive(Component)]
pub struct TagDuelObject;

#[derive(Component, Default)]
pub struct TagStatic;

#[derive(Component, Default)]
pub struct TagSemiStatic;

// Movement
#[derive(Component, Default)]
pub struct ForceAccum(pub Vec2);

#[derive(Component, Default)]
pub struct Speed(pub Vec2);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
pub struct Mass(pub f32);

#[derive(Component, Default)]
pub struct NextPosition(pub Vec3);

#[derive(Bundle)]
pub struct MoveableBundle {
    pub speed: Speed,
    pub acceleration: Acceleration,
    pub force: ForceAccum
}

impl Default for MoveableBundle {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            acceleration: Default::default(),
            force: Default::default()
        }
    }
}
