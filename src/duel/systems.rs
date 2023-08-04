use bevy::asset::AssetServer;
use bevy::prelude::*;

use super::collision::components::AABB;
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
        MoveableBundle {
            speed: Speed(100.0),
            ..default()
        },
        TagPlayer,
        TagGravity,
        AABB(Vec4::new(-16.0, -16.0, 16.0, 16.0)),
    ));

    spawn_ceiling(commands);
}

fn spawn_ceiling(mut commands: Commands) {
    commands.spawn((
        AABB(Vec4::new(-256.0, -16.0, 256.0, 16.0)),
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: -100.0,
                z: 0.0,
            },
            ..default()
        },
    ));
}
