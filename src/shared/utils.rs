use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

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