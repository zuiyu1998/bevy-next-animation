use crate::{
    assets::EntityAnimationsLoader,
    core::AnimationName,
    entity::{EntityAnimationContext, NextAnimation},
    prelude::EntityAnimations,
};
use bevy::{ecs::system::SystemState, prelude::*};

#[derive(Debug, Component)]
pub struct NextAnimationTarget {
    pub player: Entity,
}

#[derive(Default, Component)]
pub struct NextAnimationPlayer {
    pub current_animation: AnimationName,
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
        self.current_animation = AnimationName::new(animation_name);
        self.state = AnimationState::Playing;
        self.time = 0.0;
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, AnimationState::Playing)
    }

    fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    fn get_time(&self) -> f32 {
        self.time
    }
}

pub fn advance_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<&mut NextAnimationPlayer>,
    mut animation_target_q: Query<(
        &NextAnimationTarget,
        &Handle<EntityAnimations>,
        Option<&mut NextAnimation>,
        Entity,
    )>,
    animations: Res<Assets<EntityAnimations>>,
    registry: Res<AppTypeRegistry>,
    asset_server: Res<AssetServer>,
) {
    let dt = time.delta_seconds();

    let registry = registry.read();

    for (target, handle, animation, entity) in animation_target_q.iter_mut() {
        if let Ok(mut player) = player_q.get_mut(target.player) {
            if player.is_playing() {
                player.update(dt);

                let dt = player.get_time();

                if let Some(new_anmation) = NextAnimation::new(
                    &registry,
                    &asset_server,
                    &animations,
                    handle,
                    &player.current_animation,
                    dt,
                ) {
                    if let Some(mut animation) = animation {
                        *animation = new_anmation;
                    } else {
                        commands.entity(entity).insert(new_anmation);
                    }
                } else {
                    warn!("{:?} animation not found.", player.current_animation);
                }
            }
        } else {
            warn!("{} player entity not found.", target.player);
        }
    }
}

pub fn update_animations(world: &mut World) {
    let mut state = SystemState::<Query<(Entity, &NextAnimation)>>::new(world);

    let mut animations = vec![];

    let animation_q = state.get(world);

    for (entity, animation) in animation_q.iter() {
        animations.push((entity, animation.clone()));
    }

    for (entity, animation) in animations.into_iter() {
        let world = world.get_entity_mut(entity).unwrap();

        let context = EntityAnimationContext {
            animation,
            entity_world: world,
        };

        context.apply();
    }
}

pub struct BevyNextAnimationPlugin;

impl Plugin for BevyNextAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (advance_animations, update_animations)
                .chain()
                .before(TransformSystem::TransformPropagate),
        );
        app.init_asset::<EntityAnimations>()
            .init_asset_loader::<EntityAnimationsLoader>();
    }
}
