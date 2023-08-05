use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::duel::object::components::NextPosition;
use crate::shared::utils::debug_draw_box;

use super::components::{CollisionLeave, AABB};
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

pub fn register_collisions(
    query: Query<(Entity, &NextPosition, &AABB)>,
    mut event_on_collision: EventWriter<EvOnCollision>,
    mut lines: ResMut<DebugLines>,
) {
    for [a, b] in query.iter_combinations() {
        if a.0 == b.0 {
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
            let dir = a_pos - b_pos;
            let a_size = Vec2::new(a_as_box.z - a_as_box.x, a_as_box.w - a_as_box.y);
            let b_size = Vec2::new(b_as_box.z - b_as_box.x, b_as_box.w - b_as_box.y);

            let size_diff = a_size - b_size;
            let size_result = size_diff.x + size_diff.y;

            let result_entity_to_move: Entity;
            let result_vector_to_move: Vec3;
            if size_result < 0.0 {
                result_entity_to_move = a.0;
                result_vector_to_move = dir;
            } else {
                result_entity_to_move = b.0;
                result_vector_to_move = -dir;
            }

            println!("Collision {0}, {1}, {2}", a_size, b_size, size_result);

            event_on_collision.send(EvOnCollision {
                target_to_move: result_entity_to_move,
                vector_to_move: result_vector_to_move,
            });
        }
    }
}

pub fn add_collision_move(
    mut event_on_collision: EventReader<EvOnCollision>,
    mut commands: Commands,
) {
    for ev in event_on_collision.iter() {
        commands
            .entity(ev.target_to_move)
            .insert(CollisionLeave(ev.vector_to_move));
    }
}

pub fn apply_collision_move_to_force(
    mut query: Query<(Entity, &mut NextPosition, &CollisionLeave)>,
    mut commands: Commands,
) {
    for (entity, mut position, collision) in &mut query {
        position.0 += collision.0;
        commands.entity(entity).remove::<CollisionLeave>();
    }
}
