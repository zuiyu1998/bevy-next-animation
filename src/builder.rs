use crate::{assets::EntityAnimations, plugin::NextAnimationTarget};
use bevy::{prelude::*, utils::HashMap};

#[derive(Bundle)]
pub struct AnimationBundle {
    target: NextAnimationTarget,
    handle: Handle<EntityAnimations>,
}

pub struct AnimationsBuilder {
    target: Entity,
    data: HashMap<String, Handle<EntityAnimations>>,
}

impl AnimationsBuilder {
    pub fn entity(entity: Entity) -> Self {
        Self {
            target: entity,
            data: Default::default(),
        }
    }

    pub fn add_handle(&mut self, entity_class: &str, handle: Handle<EntityAnimations>) {
        self.data.insert(entity_class.to_string(), handle);
    }

    pub fn get_animation_bundle(&self, entity_class: &str) -> Option<AnimationBundle> {
        self.data.get(entity_class).and_then(|handle| {
            Some(AnimationBundle {
                target: NextAnimationTarget {
                    player: self.target,
                },
                handle: handle.clone(),
            })
        })
    }
}
