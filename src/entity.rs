use crate::track::EntityTrack;
use crate::value::BoundValueCollection;
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct ComponentShortTypePath(String);

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct AnimationName(String);

impl AnimationName {
    pub fn new(name: &str) -> Self {
        AnimationName(name.to_string())
    }
}

impl ComponentShortTypePath {
    pub fn from_type_path<T: TypePath>() -> Self {
        Self(T::short_type_path().to_string())
    }
}

#[derive(Default, Clone, Deref)]
pub struct EntityAnimation {
    pub tracks: HashMap<ComponentShortTypePath, EntityTrack>,
}

#[derive(Default, Asset, TypePath, Clone, Deref, DerefMut)]
pub struct EntityAnimations(HashMap<AnimationName, EntityAnimation>);

impl EntityAnimation {
    pub fn get_animation_pose(&self, dt: f32) -> AnimationPose {
        let mut pose = AnimationPose::default();

        for (type_path, track) in self.tracks.iter() {
            let collection = track.fetch(dt);
            pose.insert(type_path.clone(), collection);
        }

        pose
    }
}

#[derive(Deref, DerefMut, Default, Clone)]
pub struct AnimationPose(HashMap<ComponentShortTypePath, BoundValueCollection>);

impl AnimationPose {
    pub fn get_reflect_component_map(
        &self,
        registry: &AppTypeRegistry,
    ) -> HashMap<ComponentShortTypePath, ReflectComponent> {
        let mut reflect_component_map = HashMap::default();

        let registry = registry.read();

        for type_path in self.keys() {
            if let Some(registraion) = registry.get_with_short_type_path(type_path) {
                if let Some(reflect_component) = registraion.data::<ReflectComponent>() {
                    reflect_component_map.insert(type_path.clone(), reflect_component.clone());
                } else {
                    info!(
                        "type {:?} not found ReflectComponent",
                        registraion.type_info().type_path_table().ident()
                    );
                }
            }
        }

        reflect_component_map
    }
}

pub fn get_type_path<C: TypePath>() -> String {
    C::short_type_path().to_string()
}

#[derive(Component, Clone)]
pub struct NextAnimation {
    reflect_component_map: HashMap<ComponentShortTypePath, ReflectComponent>,
    pose: AnimationPose,
}

impl NextAnimation {
    pub fn new(
        registry: &AppTypeRegistry,
        animations: &Assets<EntityAnimations>,
        handle: &Handle<EntityAnimations>,
        active_name: &AnimationName,
        dt: f32,
    ) -> Option<Self> {
        animations.get(handle).and_then(|animations| {
            animations.get(active_name).and_then(|animation| {
                let pose = animation.get_animation_pose(dt);

                let reflect_component_map = pose.get_reflect_component_map(registry);

                Some(NextAnimation {
                    pose,
                    reflect_component_map,
                })
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
        for (type_apth, reflect_component) in self.animation.reflect_component_map.into_iter() {
            let collection = self.animation.pose.remove(&type_apth).unwrap();

            let component = collection.get_dynamic();

            reflect_component.apply(&mut self.entity_world, &*component);
        }
    }
}
