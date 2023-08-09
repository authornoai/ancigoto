use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

pub const GROUND_CHECK_DIR: Vec2 = Vec2::new(0.0, -1.0);

pub fn utils_ground_raycast(
    context: &Res<RapierContext>,
    ray_origin: Vec2,
    max_toi: f32,
    result_entity: &mut Entity,
    result_point: &mut Vec2,
) -> bool {
    return utils_raycast(
        context,
        ray_origin,
        GROUND_CHECK_DIR,
        max_toi,
        result_entity,
        result_point,
    );
}

pub fn utils_raycast(
    context: &Res<RapierContext>,
    ray_origin: Vec2,
    ray_dir: Vec2,
    max_toi: f32,
    result_entity: &mut Entity,
    result_point: &mut Vec2,
) -> bool {
    if let Some((entity, toi)) =
        context.cast_ray(ray_origin, ray_dir, max_toi, false, QueryFilter::default())
    {
        *result_point = ray_origin + ray_dir * toi;
        *result_entity = entity;
        return true;
    }
    return false;
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
