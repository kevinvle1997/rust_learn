//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite,movement))
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Neutral,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/running.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 5 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Direction::Up,
    ));
}

fn movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform, &mut AnimationTimer)>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    for (mut logo, mut transform, mut animation_timer) in &mut sprite_position {
        match *logo {
            Direction::Up => {transform.translation.y += 150. * time.delta_seconds(); animation_timer.0.unpause()},
            Direction::Down => {transform.translation.y -= 150. * time.delta_seconds(); animation_timer.0.unpause()},
            Direction::Left => {transform.translation.x -= 150. * time.delta_seconds(); animation_timer.0.unpause()},
            Direction::Right => {transform.translation.x += 150. * time.delta_seconds(); animation_timer.0.unpause()},
            Direction::Neutral => animation_timer.0.pause(),



        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            *logo = Direction::Down;
        } else if keyboard_input.pressed(KeyCode::KeyW) {
            *logo = Direction::Up;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            *logo = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            *logo = Direction::Right;
        } else {
            *logo = Direction::Neutral;

        }
    }

}