use bevy::asset::AssetServer;
use bevy::prelude::*;

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
        TagGravity
    ));
}
