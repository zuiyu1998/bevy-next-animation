use crate::track::EntityTrack;
use crate::value::BoundValueCollection;
use bevy::{prelude::*, utils::HashMap};

#[derive(Default, Asset, TypePath, Clone)]
pub struct EntityAnimation {
    pub tracks: HashMap<String, EntityTrack>,
}

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

#[derive(Deref, DerefMut, Default)]
pub struct AnimationPose(HashMap<String, BoundValueCollection>);

impl AnimationPose {
    pub fn get_reflect_component_map(
        &self,
        registry: &AppTypeRegistry,
    ) -> HashMap<String, ReflectComponent> {
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

pub fn update_entity_animation(
    mut entity_world: EntityWorldMut,
    registry: &AppTypeRegistry,
    mut pose: AnimationPose,
) {
    let reflect_component_map = pose.get_reflect_component_map(registry);

    for (type_id, reflect_component) in reflect_component_map.into_iter() {
        let collection = pose.remove(&type_id).unwrap();

        let component = collection.get_dynamic();

        reflect_component.apply(&mut entity_world, &*component);
    }
}
