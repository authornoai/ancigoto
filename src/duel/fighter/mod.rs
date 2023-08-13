use bevy::prelude::*;

pub mod magic_ball;
pub mod components;
mod systems;

use self::{systems::*, magic_ball::MagicBallPlugin};

pub struct FighterPlugin;

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_move_desire)
        .add_plugins(MagicBallPlugin);
    }
}
