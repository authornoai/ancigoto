use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::duel::object::components::ForceAccum;

use super::components::{CollisionLeave, AABB};
use super::events::*;

fn utils_aabb_to_vec4(aabb: &AABB, transform: &Transform) -> Vec4 {
    let global_pos = transform.translation;

    let result = Vec4::new(
        aabb.0.x + global_pos.x,
        aabb.0.y + global_pos.y,
        aabb.0.z + global_pos.x,
        aabb.0.w + global_pos.y,
    );

    return result;
}

fn utils_box_vs_box(a: Vec4, b: Vec4) -> bool {
    return a.x < b.z && b.x < a.z && a.y < b.w && b.y < a.w;
}

pub fn register_collisions(
    query: Query<(Entity, &Transform, &AABB)>,
    mut event_on_collision: EventWriter<EvOnCollision>,
    mut lines: ResMut<DebugLines>,
) {
    for [a, b] in query.iter_combinations() {
        if a.0 == b.0 {
            continue;
        }

        let a_as_box = utils_aabb_to_vec4(&a.2, &a.1);
        let b_as_box = utils_aabb_to_vec4(&b.2, &b.1);

        debug_draw_box(&mut lines, a_as_box);
        debug_draw_box(&mut lines, b_as_box);

        let result = utils_box_vs_box(a_as_box, b_as_box);
        if result {
            let a_pos = a.1.translation;
            let b_pos = b.1.translation;

            

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
                target_a: a.0,
                target_b: b.0,
                target_to_move: result_entity_to_move,
                vector_to_move: result_vector_to_move,
            });
        }
    }
}

fn debug_draw_box(lines: &mut ResMut<DebugLines>, vec_box: Vec4) {
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
    mut query: Query<(Entity, &mut ForceAccum, &CollisionLeave)>,
    mut commands: Commands,
) {
    for (entity, mut force, collision) in &mut query {
        force.0 += Vec2::new(collision.0.x, collision.0.y);
        commands.entity(entity).remove::<CollisionLeave>();
        println!("H")
    }
}
