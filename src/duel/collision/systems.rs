use bevy::prelude::*;
use bevy::{ecs::query::Has, utils::petgraph::matrix_graph::Zero};

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

    if !result.x.is_zero() && !result.y.is_zero() {
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
    while let Some([a, b]) = combinations.fetch_next() {
        let a_is_static = a.3;
        let b_is_static = b.3;

        let a_entity = a.0;
        let b_entity = b.0;

        if a_entity == b_entity || (a_is_static && b_is_static) {
            continue;
        }

        let a_pos = a.1 .0;
        let b_pos = b.1 .0;

        let a_as_box = utils_aabb_to_global_rect(&a.2, a_pos);
        let b_as_box = utils_aabb_to_global_rect(&b.2, b_pos);

        let result = utils_rect_vs_rect(a_as_box, b_as_box);
        if result {
            let a_resolution = utils_get_resolve_vector(a_as_box, b_as_box);
            let b_resolution = utils_get_resolve_vector(b_as_box, a_as_box);

            let a_size = a_as_box.size();
            let b_size = b_as_box.size();

            let target_e: Entity;
            let mut target_next_pos: Mut<NextPosition>;
            let target_resolve: Vec3;

            if b_resolution.y > 0.0 {
                target_e = b_entity;
                target_next_pos = b.1;
                target_resolve = b_resolution;
            } else if a_resolution.y > 0.0 {
                target_e = a_entity;
                target_next_pos = a.1;
                target_resolve = a_resolution;
            } else if (a_size - b_size).length_squared() > 0.0 {
                target_e = b_entity;
                target_next_pos = b.1;
                target_resolve = b_resolution;
            } else {
                target_e = a_entity;
                target_next_pos = a.1;
                target_resolve = a_resolution;
            }

            target_next_pos.0 += target_resolve;

            if target_resolve.y > 0.0 {
                commands.entity(target_e).insert(TagGrounded);
            }
        }
    }
}

pub fn remove_grounded(mut commands: Commands, query: Query<(Entity, With<TagGrounded>)>) {
    for (e, _) in &query {
        commands.entity(e).remove::<TagGrounded>();
    }
}
