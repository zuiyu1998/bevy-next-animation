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

pub fn setup(mut commands: Commands, mut entity_animation_assets: ResMut<Assets<EntityAnimation>>) {
    commands.spawn(Camera2dBundle::default());

    let mut entity_track = EntityTrack::default();

    let mut track = Track::new(
        ValueBinding {
            path: "a".to_owned(),
            value_type: ValueType::Bool,
        },
        0.5,
        2,
    );

    track.add_keyframe(Keyframe::new(1, 0.0));
    track.add_keyframe(Keyframe::new(0, 1.0));

    entity_track.add_track(track);

    let mut entity_animation = EntityAnimation::default();

    entity_animation
        .tracks
        .insert(TestA::short_type_path().to_string(), entity_track);

    let handle = entity_animation_assets.add(entity_animation);

    let entity = commands.spawn(TestA { a: false }).id();

    let mut animation = NextAnimation::default();
    animation.insert(entity, handle);

    let mut animation_player = NextAnimationPlayer::default();

    animation_player
        .animations
        .insert("test".to_string(), animation);

    animation_player.play("test");

    commands.entity(entity).insert(animation_player);
}

fn debug_test(test_a_q: Query<&TestA>) {
    for test_a in test_a_q.iter() {
        info!("{}", test_a.a);
    }
}
