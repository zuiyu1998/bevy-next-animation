//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;

use bevy_next_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            BevyNextAnimationPlugin,
        ))
        .add_systems(Startup, setup)
        .register_animate_component::<TextureAtlas>()
        .register_animate_component::<Handle<Image>>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());

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
        ))
        .id();

    let mut builder = AnimationsBuilder::entity(entity);

    let handle = asset_server.load("entity_animations/play.entity_animations.json");

    builder.add_handle("self", handle);

    let mut animation_player = NextAnimationPlayer::default();

    animation_player.play("idle");

    commands.entity(entity).insert((
        animation_player,
        builder.get_animation_bundle("self").unwrap(),
    ));
}
