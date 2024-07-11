use crate::{
    entity::{AnimationPose, EntityAnimation},
    prelude::update_entity_animation,
};
use bevy::{ecs::system::SystemState, prelude::*, utils::hashbrown::HashMap};

#[derive(Deref, Default, DerefMut)]
pub struct NextAnimation(HashMap<Entity, Handle<EntityAnimation>>);

#[derive(Default, Component)]
pub struct NextAnimationPlayer {
    pub animations: HashMap<String, NextAnimation>,
    pub current_animation: String,
    time: f32,
    state: AnimationState,
}

#[derive(Debug, Clone, Default)]
pub enum AnimationState {
    #[default]
    Reset,
    Playing,
    Stop,
}

impl NextAnimationPlayer {
    pub fn play(&mut self, animation_name: &str) {
        self.current_animation = animation_name.to_string();
        self.state = AnimationState::Playing;
        self.time = 0.0;
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, AnimationState::Playing)
    }

    pub fn get_mapper(&self) -> Option<&NextAnimation> {
        self.animations.get(&self.current_animation)
    }

    pub fn get_animation_pose(
        &mut self,
        dt: f32,
        assets: &Assets<EntityAnimation>,
    ) -> HashMap<Entity, AnimationPose> {
        if self.is_playing() {
            self.time = self.time + dt;
        }

        let mut map = HashMap::default();

        if let Some(mapper) = self.get_mapper() {
            for (entity, handle) in mapper.iter() {
                if let Some(entity_animation) = assets.get(&handle.clone_weak()) {
                    let pose = entity_animation.get_animation_pose(self.time);

                    map.insert(*entity, pose);
                }
            }
        } else {
            warn!("{} animation not found.", self.current_animation);
        }

        map
    }
}

pub fn update_animation(world: &mut World) {
    let mut state = SystemState::<(
        Query<&mut NextAnimationPlayer>,
        Res<Assets<EntityAnimation>>,
        Res<Time>,
        Res<AppTypeRegistry>,
    )>::new(world);

    let mut animations = vec![];

    let (mut mapper_query, assets, time, registry) = state.get_mut(world);

    let delta = time.delta_seconds();

    let registry = registry.clone();

    for mut mapper in mapper_query.iter_mut() {
        let animation = mapper.get_animation_pose(delta, &assets);

        animations.push(animation);
    }

    for animation in animations.into_iter() {
        for (entity, pose) in animation.into_iter() {
            let entity_world = world.entity_mut(entity);

            update_entity_animation(entity_world, &registry, pose)
        }
    }
}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animation);
        app.init_asset::<EntityAnimation>();
    }
}
