use std::{any::TypeId, marker::PhantomData};

use crate::track::EntityTrack;
use crate::value::BoundValueCollection;
use bevy::{
    ecs::system::SystemState,
    prelude::*,
    utils::{all_tuples, HashMap},
};

pub trait AnimationComponent: 'static + Sync + Send {
    fn update_world(world: EntityWorldMut, pose: AnimationPose);
}

macro_rules! impl_tuple_animation_component {
    ($($T:ident),*) => {
        impl<$($T: Reflect + Component),*>  AnimationComponent for ($(FromComponent<$T>,)*) {
            #![allow(unused)]
            fn update_world(mut world: EntityWorldMut, pose: AnimationPose) {
                $(
                    {
                        update_component::<$T>(&mut world, &pose);
                    }
                )*
            }
        }

    };
}

all_tuples!(impl_tuple_animation_component, 0, 15, T);

#[derive(Component)]
pub struct AnimationComponentMarker<A: AnimationComponent>(PhantomData<A>);

impl<A: AnimationComponent> AnimationComponentMarker<A> {
    pub fn new() -> Self {
        AnimationComponentMarker(PhantomData::default())
    }
}

pub struct FromComponent<T>(PhantomData<T>);

impl<T: Component + Reflect> AnimationComponent for FromComponent<T> {
    fn update_world(mut world: EntityWorldMut, pose: AnimationPose) {
        update_component::<T>(&mut world, &pose);
    }
}

pub fn update_component<T: Reflect + Component>(world: &mut EntityWorldMut, pose: &AnimationPose) {
    if let Some(value) = world.get_mut::<T>() {
        let relect: &mut dyn Reflect = value.into_inner();

        if let Some(collection) = pose.get(&TypeId::of::<T>()) {
            for bound_value in collection.values.iter() {
                bound_value.apply_to_object(relect);
            }
        }
    }
}

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

#[derive(Bundle)]
pub struct AnimationBundle<T: AnimationComponent> {
    player: EntityAnimationPlayer,
    marker: AnimationComponentMarker<T>,
}

impl<T: AnimationComponent> AnimationBundle<T> {
    pub fn new(player: EntityAnimationPlayer) -> Self {
        Self {
            player,
            marker: AnimationComponentMarker::new(),
        }
    }
}

pub fn update_animation<A: AnimationComponent>(world: &mut World) {
    let mut state = SystemState::<(
        Res<Time>,
        Query<
            Entity,
            (
                With<EntityAnimationPlayer>,
                With<AnimationComponentMarker<A>>,
            ),
        >,
    )>::new(world);

    let (time, animation_q) = state.get(world);

    let mut animation_entitys = vec![];
    let time: f32 = time.delta_seconds();

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

    let mut animation = entity_world.get_mut::<EntityAnimationPlayer>().unwrap();

    let pose = animation.get_animation_pose(time);

    A::update_world(entity_world, pose);
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
