use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_next_animation::{BevyNextAnimationPlugin, EntityAnimation};

#[derive(Component, Reflect, Default)]
pub struct TestA;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }),
        BevyNextAnimationPlugin::<TestA>::default(),
    ));

    app.add_systems(Startup, setup);

    app.run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let animation: EntityAnimation<TestA> = EntityAnimation::new();
    commands.spawn((TestA, animation));
}
