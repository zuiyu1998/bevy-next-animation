use crate::{
    assets::EntityAnimations, core::AnimationName, core::ComponentShortTypePath,
    track::EntityTrack, value::AnimationComponentFns, value::BoundValueCollection,
    value::ValueBinding,
};
use bevy::{
    prelude::*,
    reflect::{ReflectKind, TypeRegistry},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ReflectValueCollection {
    pub values: Vec<ReflecBoundValue>,
    pub relect_kind: ReflectKind,
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
    pub tracks: HashMap<ComponentShortTypePath, EntityTrack>,
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
            let collection = track.fetch(dt);

            if let Some(registraion) = registry.get_with_short_type_path(type_path) {
                if let Some(fns) = registraion.data::<AnimationComponentFns>() {
                    let mut values = vec![];

                    for v in collection.values.iter() {
                        let reflect = (fns.reflect)(&v.value, asset_server);

                        let bound = ReflecBoundValue {
                            value: reflect,
                            binding: v.binding.clone(),
                        };

                        values.push(bound);
                    }

                    let collection = ReflectValueCollection {
                        values,
                        relect_kind: collection.relect_kind,
                    };

                    pose.insert(type_path.clone(), collection);
                } else {
                    info!(
                        "type {:?} not found AnimationComponentFns",
                        registraion.type_info().type_path_table().ident()
                    );
                }
            }
        }

        pose
    }
}

#[derive(Deref, DerefMut, Default, Clone)]
pub struct AnimationPose(HashMap<ComponentShortTypePath, ReflectValueCollection>);

pub fn get_type_path<C: TypePath>() -> String {
    C::short_type_path().to_string()
}

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
        for (type_apth, collection) in self.animation.pose.into_iter() {

            // reflect_component.apply(&mut self.entity_world, &*component);
        }
    }
}
