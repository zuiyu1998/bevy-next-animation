use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_next_animation::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TestA {
    pub a: bool,
}

#[derive(Component, Reflect, Default)]
pub struct TestB {
    pub a: bool,
}

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
        BevyNextAnimationPlugin,
    ));

    app.register_type::<TestA>();

    app.add_systems(Startup, setup);
    app.add_systems(Update, debug_test);

    app.run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let handle = asset_server.load("entity_animations/play.entity_animations.json");

    let entity = commands.spawn(TestA { a: false }).id();

    let mut builder = AnimationsBuilder::entity(entity);
    builder.add_handle("self", handle);

    let mut animation_player = NextAnimationPlayer::default();

    animation_player.play("test");

    commands.entity(entity).insert((
        animation_player,
        builder.get_animation_bundle("self").unwrap(),
    ));
}

fn debug_test(test_a_q: Query<&TestA>) {
    for test_a in test_a_q.iter() {
        info!("{}", test_a.a);
    }
}
