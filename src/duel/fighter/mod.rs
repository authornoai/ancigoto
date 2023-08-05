use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;

pub struct FighterPlugin;

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_move_desire);
    }
}
