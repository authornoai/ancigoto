use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    duel::{collision::components::AABB, object::components::NextPosition},
    shared::utils::{debug_draw_box, utils_aabb_to_vec4},
};

pub fn draw_all_aabb(query: Query<(&NextPosition, &AABB)>, mut lines: ResMut<DebugLines>) {
    for (pos, aabb) in &query {
        debug_draw_box(&mut lines, utils_aabb_to_vec4(aabb, pos.0));
    }
}
