use bevy::prelude::*;

use super::{
    components::{MagicWind, TagCanMagicWind},
    events::EventOnWind,
};

pub fn on_wind_attack(
    query: Query<(Entity, With<TagCanMagicWind>)>,
    mut commands: Commands,
    mut event: EventReader<EventOnWind>,
) {
    for ev in event.iter() {
        for (e, _) in &query {
            commands.entity(e).insert(MagicWind(ev.dir * ev.power));
        }
        break;
    }
}
