use bevy::{prelude::*, reflect::FromType};

use super::ComponentPose;

impl<A: AnimateComponent> FromType<A> for AnimateComponentFns {
    fn from_type() -> Self {
        AnimateComponentFns {
            apply: <A as AnimateComponent>::apply,
        }
    }
}

#[derive(Clone)]
pub struct AnimateComponentFns {
    pub apply: fn(&mut EntityWorldMut, pose: ComponentPose),
}

pub trait AnimateComponent: Reflect + Component + Sized + TypePath {
    fn update(&mut self, pose: ComponentPose) {
        for value in pose.values.into_iter() {
            if let Ok(field) = self.reflect_path_mut(value.path.as_str()) {
                field.apply(&*value.value);
            }
        }
    }

    fn apply(world: &mut EntityWorldMut, pose: ComponentPose) {
        if let Some(mut v) = world.get_mut::<Self>() {
            v.update(pose);
        }
    }
}

impl<T: Reflect + Component + Sized + TypePath> AnimateComponent for T {}
