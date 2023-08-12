use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Ccd, Collider, ExternalForce, GravityScale, LockedAxes, RigidBody, Velocity,
};

use super::fighter::components::{FighterBundle, MovePower};
use super::magic::components::MagicTargetBundle;
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
            ground_raycast_pos: GroundRaycastPos {
                left_pos: Vec2::new(-7.0, -4.0),
                right_pos: Vec2::new(7.0, -4.0),
                max_toi: 4.1
            },
            ..default()
        },
        FighterBundle {
            move_power: MovePower(Vec2::new(4.0, 200.0)),
            ..default()
        },
        TagPlayer,
        Collider::capsule(Vec2::new(0.0, -4.0), Vec2::new(0.0, 4.0), 4.0),
        RigidBody::Dynamic,
        ExternalForce::default(),
        Ccd::enabled(),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(4.0),
    ));

    spawn_ceiling(commands);
}

fn spawn_ceiling(mut commands: Commands) {
    let pos = Vec3::new(0.0, -64.0, 0.0);

    commands.spawn((
        Collider::cuboid(128.0, 16.0),
        TransformBundle::from(Transform::from_translation(pos)),
    ));

    let pos = Vec3::new(64.0, 150.0, 0.0);

    commands.spawn((
        Collider::cuboid(16.0, 128.0),
        TransformBundle::from(Transform::from_translation(pos)),
    ));

    for i in 0..4 {
        let pos = Vec3::new(-32.0, 512.0 + 32.0 * i as f32, 0.0);

        commands.spawn((
            Collider::cuboid(4.0, 4.0),
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_translation(pos)),
            MoveableBundle
            {
                ground_raycast_pos: GroundRaycastPos {
                    left_pos: Vec2::new(-4.0, -2.0),
                    right_pos: Vec2::new(4.0, -2.0),
                    max_toi: 2.1
                },
                ..default()
            },
            TagMagicRigid,
            Velocity::default(),
            MagicTargetBundle::default(),
        ));
    }
}
