use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    duel::{collision::components::AABB, object::components::NextPosition},
    shared::utils::{utils_debug_draw_rect, utils_aabb_to_global_rect},
};

pub fn draw_all_aabb(query: Query<(&NextPosition, &AABB)>, mut lines: ResMut<DebugLines>) {
    for (pos, aabb) in &query {
        utils_debug_draw_rect(&mut lines, utils_aabb_to_global_rect(aabb, pos.0));
    }
}
