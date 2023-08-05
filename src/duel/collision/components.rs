use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AABB(pub Rect);

#[derive(Component, Default)]
pub struct TagGrounded;