use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_next_animation::{
    track::{EntityTrack, Keyframe, Track},
    value::{ValueBinding, ValueType},
    AnimationBundle, BevyNextAnimationPlugin, EntityAnimationPlayer,
};

#[derive(Component, Reflect, Default)]
pub struct TestA {
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
        BevyNextAnimationPlugin::<TestA>::new(),
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, debug_test);

    app.run();
}

pub fn setup(mut commands: Commands) {
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

    let mut animation_player = EntityAnimationPlayer::new();

    animation_player.add_entity_track::<TestA>(entity_track);

    animation_player.playing();

    commands.spawn((
        TestA { a: false },
        AnimationBundle::<TestA>::new(animation_player),
    ));
}

fn debug_test(test_a_q: Query<&TestA>) {
    for test_a in test_a_q.iter() {
        info!("{}", test_a.a);
    }
}
