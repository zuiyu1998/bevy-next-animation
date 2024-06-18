pub mod value;

use bevy::prelude::*;

#[derive(Component)]
pub struct EntityTrack {}

pub fn update_animation() {}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation);
    }
}
