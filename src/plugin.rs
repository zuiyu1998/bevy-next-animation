use std::any::TypeId;

use crate::track::EntityTrack;
use crate::value::BoundValueCollection;
use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};

#[derive(Component)]
pub struct EntityAnimationPlayer {
    animation: EntityAnimation,

    time: f32,
    state: EntityAnimationState,
}

#[derive(Default)]
pub struct EntityAnimation {
    pub tracks: HashMap<TypeId, EntityTrack>,
}

pub enum EntityAnimationState {
    Playing,
    Stop,
    Reset,
}

impl EntityAnimationPlayer {
    pub fn new() -> Self {
        EntityAnimationPlayer {
            time: 0.0,
            state: EntityAnimationState::Reset,
            animation: Default::default(),
        }
    }

    pub fn playing(&mut self) {
        self.state = EntityAnimationState::Playing;
    }

    pub fn stop(&mut self) {
        self.state = EntityAnimationState::Stop;
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, EntityAnimationState::Playing)
    }

    pub fn reset(&mut self) {
        self.state = EntityAnimationState::Reset;
        self.time = 0.0;
    }

    pub fn add_entity_track<T: Reflect + Component>(&mut self, track: EntityTrack) {
        self.animation.tracks.insert(TypeId::of::<T>(), track);
    }

    pub fn get_mut_entity_track<T: Reflect + Component>(&mut self) -> Option<&mut EntityTrack> {
        self.animation.tracks.get_mut(&TypeId::of::<T>())
    }

    pub fn get_entity_track<T: Reflect + Component>(&mut self) -> Option<&EntityTrack> {
        self.animation.tracks.get(&TypeId::of::<T>())
    }

    pub fn get_animation_pose(&mut self, dt: f32) -> AnimationPose {
        if self.is_playing() {
            self.time = self.time + dt;
        }

        let mut pose = AnimationPose::default();

        for (type_id, track) in self.animation.tracks.iter() {
            let collection = track.fetch(self.time);
            pose.insert(*type_id, collection);
        }

        pose
    }
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

pub fn update_animation(world: &mut World) {
    let mut state = SystemState::<(
        Res<Time>,
        Res<AppTypeRegistry>,
        Query<Entity, (With<EntityAnimationPlayer>,)>,
    )>::new(world);

    let (time, registry, animation_q) = state.get(world);

    let registry = registry.clone();

    let mut animation_entitys = vec![];
    let time: f32 = time.delta_seconds();

    for entity in animation_q.iter() {
        animation_entitys.push(entity);
    }

    for entity in animation_entitys.into_iter() {
        update_entity_animation(world, entity, time, &registry);
    }
}

pub fn update_entity_animation(
    world: &mut World,
    entity: Entity,
    time: f32,
    registry: &AppTypeRegistry,
) {
    let mut entity_world = world.get_entity_mut(entity).unwrap();

    let mut animation = entity_world.get_mut::<EntityAnimationPlayer>().unwrap();

    let mut pose = animation.get_animation_pose(time);

    let reflect_component_map = pose.get_reflect_component_map(registry);

    for (type_id, reflect_component) in reflect_component_map.into_iter() {
        let collection = pose.remove(&type_id).unwrap();

        let component = collection.get_dynamic();

        reflect_component.apply(&mut entity_world, &*component);
    }
}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation);
    }
}
