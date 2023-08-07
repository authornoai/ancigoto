use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBody, Collider, Ccd, LockedAxes, ExternalImpulse};

use super::fighter::components::{FighterBundle, MovePower};
use super::object::components::*;
use super::player::components::TagPlayer;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        MoveableBundle::default(),
        FighterBundle {
            move_power: MovePower(Vec2::new(0.1, 3.0)),
            ..default()
        },
        TagPlayer,
        Collider::cuboid(8.0, 8.0),
        RigidBody::Dynamic,
        ExternalImpulse::default(),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
    ));

    spawn_ceiling(commands);
}

fn spawn_ceiling(mut commands: Commands) {
    let pos = Vec3::new(0.0, -64.0, 0.0);

    commands.spawn((
        Collider::cuboid(128.0, 16.0),
        TransformBundle::from(Transform::from_translation(pos))
    ));

    let pos = Vec3::new(64.0, 150.0, 0.0);

    commands.spawn((
        Collider::cuboid(16.0, 128.0),
        TransformBundle::from(Transform::from_translation(pos))
    ));

    for i in 0..4 {
        let pos = Vec3::new(-32.0, 512.0 + 32.0 * i as f32, 0.0);

        commands.spawn((
            Collider::cuboid(4.0, 4.0),
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_translation(pos)),
            MoveableBundle::default(),
        ));
    }
}
