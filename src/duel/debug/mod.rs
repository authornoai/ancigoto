use bevy::prelude::*;

use self::systems::*;

mod systems;

pub struct DuelDebugPlugin;

impl Plugin for DuelDebugPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, draw_all_aabb);
    }
}