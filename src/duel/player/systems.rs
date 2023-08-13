use bevy::{ecs::query::Has, prelude::*};

use super::components::TagPlayer;
use crate::duel::{
    fighter::components::{MoveVec, TagInSpellcast}, magic::events::EventOnWind, object::components::TagGrounded,
};

pub fn handle_movement_input(
    mut query: Query<(&mut MoveVec, Has<TagGrounded>, With<TagPlayer>)>,
    input: Res<Input<KeyCode>>,
) {
    for (mut move_vec, has_ground, _) in &mut query {
        let mut movement_dir = Vec2::ZERO;

        if input.just_pressed(KeyCode::Space) && has_ground {
            movement_dir.y = 1.0;
        } else if input.pressed(KeyCode::S) {
            //movement_dir.y = -1.0;
        }

        if input.pressed(KeyCode::A) {
            movement_dir.x = -1.0;
        } else if input.pressed(KeyCode::D) {
            movement_dir.x = 1.0;
        }

        move_vec.0 = movement_dir;
    }
}

pub fn handle_magic_attack(input: Res<Input<MouseButton>>, mut wind: EventWriter<EventOnWind>) {
    if input.just_pressed(MouseButton::Left) {
        wind.send(EventOnWind { pos: Vec2::ZERO, dir: Vec2::X, power: 32.0 });
    }
}

pub fn handle_magic_creation(input: Res<Input<MouseButton>>,
mut commands: Commands,
player: Query<(Entity, With<TagPlayer>)>)
{
    let player = player.get_single().expect("No player!");

    if input.just_pressed(MouseButton::Right)
    {
        commands.entity(player.0).insert(TagInSpellcast);
    } else if input.just_released(MouseButton::Right)
    {
        commands.entity(player.0).remove::<TagInSpellcast>();
    }
}
