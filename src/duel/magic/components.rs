use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MagicWind(pub Vec2);

#[derive(Component, Default)]
pub struct TagCanMagicWind;


#[derive(Bundle)]
pub struct MagicTargetBundle {
    pub can_wind: TagCanMagicWind,
}

impl Default for MagicTargetBundle {
    fn default() -> Self {
        MagicTargetBundle {
            can_wind: Default::default(),
        }
    }
}
