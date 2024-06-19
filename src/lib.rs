pub mod value;

use bevy::{
    ecs::{component::ComponentId, system::SystemState},
    prelude::*,
    utils::HashMap,
};
use value::BoundValueCollection;

#[derive(Component)]
pub struct EntityAnimation {
    values: HashMap<ComponentId, BoundValueCollection>,
}

impl EntityAnimation {
    pub fn get_animation_pose(&mut self, time: f32) -> AnimationPose {
        todo!()
    }
}

#[derive(Deref, DerefMut)]
pub struct AnimationPose(HashMap<ComponentId, BoundValueCollection>);

pub fn update_animation(world: &mut World) {
    let mut state = SystemState::<(Res<Time>, Query<Entity, With<EntityAnimation>>)>::new(world);

    let (time, animation_q) = state.get(world);

    let mut animation_entitys = vec![];
    let time = time.elapsed_seconds();

    for entity in animation_q.iter() {
        animation_entitys.push(entity);
    }
}

pub fn update_entity_animation(world: &mut World, entity: Entity, time: f32) {
    let mut enitity_ref = world.get_entity_mut(entity).unwrap();

    let mut animation = enitity_ref.get_mut::<EntityAnimation>().unwrap();

    let pose = animation.get_animation_pose(time);

    for (component_id, bound_value_collection) in pose.iter() {
        if let Some(mut ptr) = enitity_ref.get_mut_by_id(*component_id) {
            let reflect = ptr.into_inner().as_ptr();

            for bind_value in bound_value_collection.values.iter() {}
        }
    }
}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation);
    }
}
