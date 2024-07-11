use std::any::TypeId;

use crate::value::BoundValueCollection;
use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};

pub fn update_animation(world: &mut World) {}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation);
    }
}
