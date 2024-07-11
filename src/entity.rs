use crate::track::EntityTrack;
use crate::value::BoundValueCollection;
use bevy::{prelude::*, utils::HashMap};
use std::any::TypeId;

#[derive(Default)]
pub struct EntityAnimation {
    pub tracks: HashMap<TypeId, EntityTrack>,
}

#[derive(Deref, DerefMut, Default)]
pub struct AnimationPose(HashMap<TypeId, BoundValueCollection>);

impl AnimationPose {
    pub fn get_reflect_component_map(
        &self,
        registry: &AppTypeRegistry,
    ) -> HashMap<TypeId, ReflectComponent> {
        let mut reflect_component_map = HashMap::default();

        let registry = registry.read();

        for type_id in self.keys() {
            if let Some(registraion) = registry.get(type_id.clone()) {
                if let Some(reflect_component) = registraion.data::<ReflectComponent>() {
                    reflect_component_map.insert(type_id.clone(), reflect_component.clone());
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
