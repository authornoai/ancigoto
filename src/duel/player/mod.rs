use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_movement_input, handle_magic_attack, handle_magic_creation));
    }
}
