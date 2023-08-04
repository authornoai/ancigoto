use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AABB(pub Vec4);

#[derive(Component, Default)]
pub struct AABBTolerant(pub Vec4);