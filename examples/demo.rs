use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_next_animation::prelude::*;

#[derive(Reflect, Component)]
pub struct TestA {
    a: bool,
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
    app.register_animate_component::<TestA>();

    app.add_systems(Startup, setup);
    app.add_systems(Update, debug_print);

    app.run();
}

pub fn setup(
    mut commands: Commands,
    mut entity_animations_assets: ResMut<Assets<EntityAnimations>>,
) {
    commands.spawn(Camera2dBundle::default());

    let binding = ValueBinding {
        path: ".a".to_owned(),
        component_type: ShortTypePath::from_type_path::<bool>(),
    };

    let mut track = Track::new(binding, 0.1, 2);

    track.add_keyframe(Keyframe::new(0, TrackValue::Number(0.0)));
    track.add_keyframe(Keyframe::new(1, TrackValue::Number(1.0)));

    let mut entity_track = ComponentTrack::default();

    entity_track.add_track(track);

    let mut entity_animation = EntityAnimation::default();

    entity_animation
        .tracks
        .insert(ShortTypePath::from_type_path::<TestA>(), entity_track);

    let mut entity_animations = EntityAnimations::default();
    entity_animations.insert(AnimationName::new("idle"), entity_animation);

    println!("{}", serde_json::to_string(&entity_animations).unwrap());

    let entity = commands.spawn(TestA { a: false }).id();

    let mut builder = AnimationsBuilder::entity(entity);

    builder.add_handle("self", entity_animations_assets.add(entity_animations));

    let mut animation_player = NextAnimationPlayer::default();

    animation_player.play("idle");

    commands.entity(entity).insert((
        animation_player,
        builder.get_animation_bundle("self").unwrap(),
    ));
}

fn debug_print(query: Query<&TestA>) {
    for test_a in query.iter() {
        println!("{}", test_a.a);
    }
}
