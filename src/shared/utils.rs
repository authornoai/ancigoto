use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

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