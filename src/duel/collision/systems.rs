use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::duel::object::components::{NextPosition, TagStatic};
use crate::shared::utils::debug_draw_box;

use super::components::AABB;
use super::events::*;

fn utils_aabb_to_vec4(aabb: &AABB, position: Vec3) -> Vec4 {
    let result = Vec4::new(
        aabb.0.x + position.x,
        aabb.0.y + position.y,
        aabb.0.z + position.x,
        aabb.0.w + position.y,
    );

    return result;
}

fn utils_box_vs_box(a: Vec4, b: Vec4) -> bool {
    return a.x < b.z && b.x < a.z && a.y < b.w && b.y < a.w;
}

fn utils_get_resolve_vector(a: Vec4, b: Vec4) -> Vec3 {
    let mut result = Vec3::ZERO;

    if a.x < b.x {
        result.x = b.x - a.z
    } else if a.z > b.z {
        result.x = b.z - a.x
    }

    if a.y < b.y {
        result.y = b.y - a.w
    } else if a.w > b.w {
        result.y = b.w - a.y
    }

    return result;
}

pub fn handle_collisions(
    mut query: Query<(Entity, &mut NextPosition, &AABB, Has<TagStatic>)>,
    mut lines: ResMut<DebugLines>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut a, mut b]) = combinations.fetch_next() {
        if a.0 == b.0 || (a.3 && b.3) {
            continue;
        }

        let a_pos = a.1 .0;
        let b_pos = b.1 .0;

        let a_as_box = utils_aabb_to_vec4(&a.2, a_pos);
        let b_as_box = utils_aabb_to_vec4(&b.2, b_pos);

        debug_draw_box(&mut lines, a_as_box);
        debug_draw_box(&mut lines, b_as_box);

        let result = utils_box_vs_box(a_as_box, b_as_box);
        if result {
            //TODO: Mass comprassion and percentages
            let a_size = Vec2::new(a_as_box.z - a_as_box.x, a_as_box.w - a_as_box.y);
            let b_size = Vec2::new(b_as_box.z - b_as_box.x, b_as_box.w - b_as_box.y);

            let size_diff = a_size - b_size;
            let size_result = size_diff.x + size_diff.y;

            if size_result < 0.0 {
                a.1 .0 += utils_get_resolve_vector(a_as_box, b_as_box);
            } else {
                b.1 .0 += utils_get_resolve_vector(b_as_box, a_as_box);
            }

            println!("Collision {0}, {1}, {2}", a_size, b_size, size_result);
        }
    }
}
