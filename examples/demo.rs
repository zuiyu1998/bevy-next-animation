use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_next_animation::prelude::*;

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

    app.register_type::<TextureAtlas>();

    app.add_systems(Startup, setup);
    app.add_systems(Update, animate_sprite);

    app.run();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct AnimationIndices {
    index: usize,
}

fn animate_sprite(mut query: Query<(&AnimationIndices, &mut TextureAtlas)>) {
    for (indices, mut atlas) in &mut query {
        atlas.index = indices.index;
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    println!("{}", get_type_path::<TextureAtlas>());

    let handle = asset_server.load("entity_animations/play.entity_animations.json");

    let entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(6.0)),
                texture,
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            AnimationIndices { index: 0 },
        ))
        .id();

    let mut builder = AnimationsBuilder::entity(entity);
    builder.add_handle("self", handle);

    let mut animation_player = NextAnimationPlayer::default();

    animation_player.play("idle");

    commands.entity(entity).insert((
        animation_player,
        builder.get_animation_bundle("self").unwrap(),
    ));
}
