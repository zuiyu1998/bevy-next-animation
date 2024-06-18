pub mod value;

use bevy::{ecs::component::ComponentId, prelude::*, utils::HashMap};
use value::BoundValueCollection;

#[derive(Component)]
pub struct EntityTrack {
    values: HashMap<ComponentId, BoundValueCollection>,
}

pub fn update_animation(world: &mut World) {
    let mut tracks = world.query_filtered::<&EntityTrack, With<EntityTrack>>();

    for track in tracks.iter(world) {}
}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation);
    }
}
