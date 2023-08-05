use bevy::asset::AssetServer;
use bevy::prelude::*;

use super::collision::components::AABB;
use super::fighter::components::{FighterBundle, MovePower};
use super::gravitation::components::TagGravity;
use super::object::components::*;
use super::player::components::TagPlayer;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        MoveableBundle { ..default() },
        FighterBundle {
            move_power: MovePower(Vec2::new(25.0, 600.0)),
            ..default()
        },
        TagPlayer,
        TagGravity,
        AABB(Vec4::new(-8.0, -8.0, 8.0, 8.0)),
        NextPosition::default(),
    ));

    spawn_ceiling(commands);
}

fn spawn_ceiling(mut commands: Commands) {
    let pos = Vec3::new(0.0, -64.0, 0.0);

    commands.spawn((
        AABB(Vec4::new(-256.0, -16.0, 256.0, 16.0)),
        Transform {
            translation: pos,
            ..default()
        },
        TagStatic,
        NextPosition(pos),
    ));

    let pos_b = Vec3::new(64.0, 256.0, 0.0);

    commands.spawn((
        AABB(Vec4::new(-16.0, -256.0, 16.0, 256.0)),
        Transform {
            translation: pos_b,
            ..default()
        },
        TagStatic,
        NextPosition(pos_b),
    ));
}
