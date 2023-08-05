use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::duel::collision::components::AABB;

pub fn utils_aabb_to_global_rect(aabb: &AABB, position: Vec3) -> Rect {
    let pos_2d = Vec2::new(position.x, position.y);
    let result = Rect::from_corners(aabb.0.min + pos_2d, aabb.0.max + pos_2d);

    return result;
}

pub fn utils_debug_draw_rect(lines: &mut ResMut<DebugLines>, rect: Rect) {
    lines.line(
        Vec3::new(rect.min.x, rect.min.y, 0.0),
        Vec3::new(rect.max.x, rect.min.y, 0.0),
        0.0,
    );

    lines.line(
        Vec3::new(rect.max.x, rect.min.y, 0.0),
        Vec3::new(rect.max.x, rect.max.y, 0.0),
        0.0,
    );

    lines.line(
        Vec3::new(rect.max.x, rect.max.y, 0.0),
        Vec3::new(rect.min.x, rect.max.y, 0.0),
        0.0,
    );

    lines.line(
        Vec3::new(rect.min.x, rect.max.y, 0.0),
        Vec3::new(rect.min.x, rect.min.y, 0.0),
        0.0,
    );

}