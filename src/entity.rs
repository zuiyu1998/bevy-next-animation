use crate::{
    assets::EntityAnimations,
    core::{AnimationName, ShortTypePath},
    prelude::AnimateComponentFns,
    track::ComponentTrack,
    value::ValueBinding,
};
use bevy::{prelude::*, reflect::TypeRegistry, utils::HashMap};
use serde::{Deserialize, Serialize};

pub struct ReflectComponent {
    pub reflect: Box<dyn Reflect>,
    pub apply: AnimateComponentFns,
}

impl Clone for ReflectComponent {
    fn clone(&self) -> Self {
        ReflectComponent {
            reflect: self.reflect.clone_value(),
            apply: self.apply.clone(),
        }
    }
}

pub struct ReflecBoundValue {
    pub binding: ValueBinding,
    pub value: Option<Box<dyn Reflect>>,
}

impl Clone for ReflecBoundValue {
    fn clone(&self) -> Self {
        ReflecBoundValue {
            binding: self.binding.clone(),
            value: self
                .value
                .as_ref()
                .and_then(|value| Some(value.clone_value())),
        }
    }
}

#[derive(Default, Clone, Deref, Deserialize, Serialize)]
pub struct EntityAnimation {
    pub tracks: HashMap<ShortTypePath, ComponentTrack>,
}

impl EntityAnimation {
    pub fn get_animation_pose(
        &self,
        dt: f32,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> AnimationPose {
        let mut pose = AnimationPose::default();

        for (type_path, track) in self.tracks.iter() {
            if let Some(registraion) = registry.get_with_short_type_path(&type_path) {
                if let Some(apply) = registraion.data::<AnimateComponentFns>() {
                    if let Some(collection) = track.fetch(dt) {
                        if let Some(reflect) = collection.get_dynamic(registry, asset_server) {
                            pose.insert(
                                type_path.clone(),
                                ReflectComponent {
                                    reflect,
                                    apply: apply.clone(),
                                },
                            );
                        }
                    }
                } else {
                    warn!("{:?} not register_animate_component.", type_path);
                }
            } else {
                warn!("{:?} not register_type.", type_path);
            }
        }

        pose
    }
}

#[derive(Deref, DerefMut, Default, Clone)]
pub struct AnimationPose(pub HashMap<ShortTypePath, ReflectComponent>);

#[derive(Component, Clone)]
pub struct NextAnimation {
    pose: AnimationPose,
}

impl NextAnimation {
    pub fn new(
        registry: &TypeRegistry,
        asset_server: &AssetServer,
        animations: &Assets<EntityAnimations>,
        handle: &Handle<EntityAnimations>,
        active_name: &AnimationName,
        dt: f32,
    ) -> Option<Self> {
        animations.get(handle).and_then(|animations| {
            animations.get(active_name).and_then(|animation| {
                let pose = animation.get_animation_pose(dt, registry, asset_server);

                Some(NextAnimation { pose })
            })
        })
    }
}

pub struct EntityAnimationContext<'a> {
    pub entity_world: EntityWorldMut<'a>,
    pub animation: NextAnimation,
}

impl<'a> EntityAnimationContext<'a> {
    pub fn apply(mut self) {
        for (_, component) in self.animation.pose.0.into_iter() {
            (component.apply.apply)(&mut self.entity_world, component.reflect);
        }
    }
}
