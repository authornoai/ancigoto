use bevy::{ecs::query::Has, utils::petgraph::matrix_graph::Zero};
use bevy::prelude::*;

use super::components::AABB;
use crate::{
    duel::{
        collision::components::TagGrounded,
        object::components::{NextPosition, TagStatic},
    },
    shared::utils::utils_aabb_to_global_rect,
};

fn utils_rect_vs_rect(a: Rect, b: Rect) -> bool {
    return a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y;
}

fn utils_get_resolve_vector(a: Rect, b: Rect) -> Vec3 {
    let mut result = Vec3::ZERO;

    if a.min.x < b.min.x {
        result.x = b.min.x - a.max.x
    } else if a.max.x > b.max.x {
        result.x = b.max.x - a.min.x
    }

    if a.min.y < b.min.y {
        result.y = b.min.y - a.max.y
    } else if a.max.y > b.max.y {
        result.y = b.max.y - a.min.y
    }

    if !result.x.is_zero() && !result.y.is_zero()
    {
        if result.y.abs() > result.x.abs() {
            result.y = 0.0;
        } else {
            result.x = 0.0;
        }
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

        let a_as_box = utils_aabb_to_global_rect(&a.2, a_pos);
        let b_as_box = utils_aabb_to_global_rect(&b.2, b_pos);

        let result = utils_rect_vs_rect(a_as_box, b_as_box);
        if result {
            //TODO: Mass comprassion and percentages
            let a_size = a_as_box.size();
            let b_size = b_as_box.size();

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
    box_main: Rect,
    box_second: Rect,
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
