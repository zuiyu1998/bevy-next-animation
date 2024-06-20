pub mod value;

use std::{any::TypeId, marker::PhantomData};

use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};
use value::{BoundValueCollection, ReflectCollection};

pub trait AnimationComponent: Component + Reflect {
    fn get_reflects(world: EntityWorldMut) -> ReflectCollection;
}

impl<T: Component + Reflect> AnimationComponent for T {
    fn get_reflects(mut world: EntityWorldMut) -> ReflectCollection {
        let mut reflect_collection = ReflectCollection::default();

        if let Some(value) = world.get_mut::<T>() {
            let reflect: Box<dyn Reflect> = unsafe { Box::from_raw(value.into_inner()) };

            reflect_collection.values.insert(TypeId::of::<T>(), reflect);
        }

        reflect_collection
    }
}

#[derive(Component)]
pub struct EntityAnimation<A> {
    _marker: PhantomData<A>,
}

impl<A> EntityAnimation<A> {
    pub fn new() -> Self {
        EntityAnimation {
            _marker: PhantomData::default(),
        }
    }

    pub fn get_animation_pose(&mut self, _time: f32) -> AnimationPose {
        let pose = AnimationPose::default();

        pose
    }
}

#[derive(Deref, DerefMut, Default)]
pub struct AnimationPose(HashMap<TypeId, BoundValueCollection>);

pub fn update_animation<A: AnimationComponent>(world: &mut World) {
    let mut state = SystemState::<(Res<Time>, Query<Entity, With<EntityAnimation<A>>>)>::new(world);

    let (time, animation_q) = state.get(world);

    let mut animation_entitys = vec![];
    let time: f32 = time.elapsed_seconds();

    for entity in animation_q.iter() {
        animation_entitys.push(entity);
    }

    for entity in animation_entitys.into_iter() {
        update_entity_animation::<A>(world, entity, time);
    }
}

pub fn update_entity_animation<A: AnimationComponent>(
    world: &mut World,
    entity: Entity,
    time: f32,
) {
    let mut entity_world = world.get_entity_mut(entity).unwrap();

    let mut animation = entity_world.get_mut::<EntityAnimation<A>>().unwrap();

    let pose = animation.get_animation_pose(time);

    let mut reflect_collection = A::get_reflects(entity_world);

    for (type_id, bound_value_conllection) in pose.iter() {
        if let Some(mut reflect) = reflect_collection.values.remove(type_id) {
            let reflect = &mut (*reflect);

            for bound_value in bound_value_conllection.values.iter() {
                bound_value.apply_to_object(reflect);
            }
        }
    }
}

pub struct BevyNextAnimationPlugin<A>(PhantomData<A>);

impl<A> BevyNextAnimationPlugin<A> {
    pub fn new() -> Self {
        BevyNextAnimationPlugin(PhantomData::default())
    }
}

impl<A: AnimationComponent> Plugin for BevyNextAnimationPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation::<A>);
    }
}
