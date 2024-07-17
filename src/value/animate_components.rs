use bevy::{prelude::*, reflect::FromType};

impl<A: AnimateComponent> FromType<A> for AnimateComponentFns {
    fn from_type() -> Self {
        AnimateComponentFns {
            apply: <A as AnimateComponent>::apply,
        }
    }
}

pub trait AnimateComponent: Reflect + Component + Sized + TypePath {
    fn apply(world: &mut EntityWorldMut, value: Box<dyn Reflect>) {
        if let Some(mut v) = world.get_mut::<Self>() {
            v.apply(&*value);
        }
    }
}

impl<T: Reflect + Component + Sized + TypePath> AnimateComponent for T {}

#[derive(Clone)]
pub struct AnimateComponentFns {
    pub apply: fn(&mut EntityWorldMut, Box<dyn Reflect>),
}
