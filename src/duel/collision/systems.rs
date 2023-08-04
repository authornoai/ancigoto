use bevy::prelude::*;

use super::components::AABB;
use super::events::OnCollision;

fn utils_aabb_to_vec4(aabb: &AABB, transform: &Transform) -> Vec4 {
    let global_pos = transform.translation;

    let result = Vec4::new(
        aabb.0.x + global_pos.x,
        aabb.0.y + global_pos.y,
        aabb.0.z + global_pos.x,
        aabb.0.w + global_pos.y,
    );
    println!("Box:{}", result);

    return result;
}

fn utils_check_in_line(a: Vec2, b: Vec2) -> bool {
    return (a.x > b.x && a.x < b.y) || (a.y > b.x && a.y < b.y);
    //return a.x > b.x && a.x < b.z && a.y > b.y && a.y < b.w;
}

fn utils_box_vs_box(a: Vec4, b: Vec4) -> bool {
    let x_gap = Vec2::new(b.x, b.z);
    let y_gap = Vec2::new(b.y, b.w);

    let has_x = utils_check_in_line(Vec2::new(a.x, a.z), x_gap);
    let has_y = utils_check_in_line(Vec2::new(a.y, a.w), y_gap);

    if !has_x && !has_y {
        return false;
    }
    return true;
}

pub fn register_collisions(
    query: Query<(Entity, &Transform, &AABB)>,
    mut event_on_collision: EventWriter<OnCollision>,
) {
    for [a, b] in query.iter_combinations() {
        if a.0 == b.0 {
            continue;
        }

        let b_as_box = utils_aabb_to_vec4(&b.2, &b.1);
        let a_as_box = utils_aabb_to_vec4(&a.2, &a.1);

        let result = utils_box_vs_box(a_as_box, b_as_box);
        if result {
            event_on_collision.send(OnCollision {
                target_a: a.0,
                target_b: b.0,
            });

            println!("Collision");
        }
    }
}
