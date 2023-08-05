use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn setup(
    mut commands: Commands
){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin 
    {
        min_width: 256.0,
        min_height: 144.0
    };

    commands.spawn(camera);
}