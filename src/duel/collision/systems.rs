use bevy::ecs::query::Has;
use bevy::prelude::*;

use super::components::AABB;
use crate::{
    duel::{
        collision::components::TagGrounded,
        object::components::{NextPosition, TagStatic},
    },
    shared::utils::utils_aabb_to_vec4,
};

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
    mut commands: Commands,
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

        let result = utils_box_vs_box(a_as_box, b_as_box);
        if result {
            //TODO: Mass comprassion and percentages
            let a_size = Vec2::new(a_as_box.z - a_as_box.x, a_as_box.w - a_as_box.y);
            let b_size = Vec2::new(b_as_box.z - b_as_box.x, b_as_box.w - b_as_box.y);

            let size_diff = a_size - b_size;
            let size_result = size_diff.x + size_diff.y;

            if size_result < 0.0 {
                handle_resolution(a_as_box, b_as_box, &mut a, &mut commands);
            } else {
                handle_resolution(b_as_box, a_as_box, &mut b, &mut commands);
            }

            println!("Collision {0}, {1}, {2}", a_size, b_size, size_result);
        }
    }
}

fn handle_resolution(
    box_main: Vec4,
    box_second: Vec4,
    entity_main: &mut (Entity, Mut<'_, NextPosition>, &AABB, bool),
    commands: &mut Commands,
) {
    let resolve = utils_get_resolve_vector(box_main, box_second);
    entity_main.1 .0 += resolve;

    if resolve.y > 0.0 {
        commands.entity(entity_main.0).insert(TagGrounded);
    }
}

pub fn remove_grounded(mut commands: Commands, query: Query<(Entity, With<TagGrounded>)>) {
    for (e, _) in &query {
        commands.entity(e).remove::<TagGrounded>();
    }
}
