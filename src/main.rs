mod components;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Office Nindou".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, (spawn_pig, pig_lifetime))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin 
    {
        min_width: 256.0,
        min_height: 144.0
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        let mut movement_dir: Vec2 = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::W) {
            movement_dir.y = 1.0;
        } else if input.pressed(KeyCode::S) {
            movement_dir.y = -1.0;
        }

        if input.pressed(KeyCode::A) {
            movement_dir.x = -1.0;
        } else if input.pressed(KeyCode::D) {
            movement_dir.x = 1.0;
        }

        movement_dir *= movement_amount;

        transform.translation += Vec3{x: movement_dir.x, y: movement_dir.y, z: 0.0};
    }
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>
)
{
    if !input.just_pressed(KeyCode::Space)
    {
        return;
    }

    let player_tranform = player.single();

    if money.0 >= 10.0 {
        money.0 -=10.0;
        info!("spent $10 on a pig, remaining money: ${:?}", money.0);

        let texture = asset_server.load("pig.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_tranform,
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
)
{
    for(pig_entity, mut pig) in &mut pigs
    {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() 
        {
            money.0 += 15.0;

            commands.entity(pig_entity).despawn();

            info!("Pig sold for $15! Current Money: ${:?}", money.0);
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer
}

#[derive(Resource)]
pub struct Money(pub f32);