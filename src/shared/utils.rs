use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::duel::collision::components::AABB;

pub fn utils_aabb_to_vec4(aabb: &AABB, position: Vec3) -> Vec4 {
    let result = Vec4::new(
        aabb.0.x + position.x,
        aabb.0.y + position.y,
        aabb.0.z + position.x,
        aabb.0.w + position.y,
    );

    return result;
}

pub fn debug_draw_box(lines: &mut ResMut<DebugLines>, vec_box: Vec4) {
    lines.line(
        Vec3::new(vec_box.x, vec_box.y, 0.0),
        Vec3::new(vec_box.z, vec_box.y, 0.0),
        0.0,
    );

    lines.line(
        Vec3::new(vec_box.x, vec_box.w, 0.0),
        Vec3::new(vec_box.z, vec_box.w, 0.0),
        0.0,
    );

    lines.line(
        Vec3::new(vec_box.x, vec_box.y, 0.0),
        Vec3::new(vec_box.x, vec_box.w, 0.0),
        0.0,
    );

    lines.line(
        Vec3::new(vec_box.z, vec_box.y, 0.0),
        Vec3::new(vec_box.z, vec_box.w, 0.0),
        0.0,
    );
}