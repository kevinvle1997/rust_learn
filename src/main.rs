use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite,movement))
        .run();
}

#[derive(Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, PartialEq, Eq, Copy)]
enum PlayerState {
    Running,
    Neutral,
    Jumping,
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
    let texture = asset_server.load("sprites/person.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 2, None, None);
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
        PlayerState::Neutral,
    ));
}

fn movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, &mut AnimationTimer, &mut AnimationIndices, &mut PlayerState)>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for (mut transform, mut animation_timer, mut animation_indices, mut player_state) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::KeyA) {
            animation_indices.first = 0;
            animation_indices.last = 5;
            *player_state = PlayerState::Running;
            transform.translation.x -= 150. * time.delta_seconds();
            transform.scale.x = -6.0; // Flip the sprite horizontally
            animation_timer.0.unpause();
        } if keyboard_input.pressed(KeyCode::KeyD) {
            animation_indices.first = 0;
            animation_indices.last = 5;
            *player_state = PlayerState::Running;
            transform.translation.x += 150. * time.delta_seconds();
            animation_timer.0.unpause();
            transform.scale.x = 6.0; // Flip the sprite horizontally
        } if keyboard_input.pressed(KeyCode::Space) {
            jump(time, &mut transform, &mut animation_timer, &mut animation_indices, player_state);
        } if !keyboard_input.pressed(KeyCode::KeyA) && !keyboard_input.pressed(KeyCode::KeyD) || (keyboard_input.pressed(KeyCode::KeyD) && keyboard_input.pressed(KeyCode::KeyA)){
            *player_state = PlayerState::Neutral;
        }

        if *player_state == PlayerState::Neutral {
            animation_timer.0.pause();
        }
    }
}

fn jump(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut AnimationTimer, &mut AnimationIndices, &mut PlayerState)>,
) {
    for (mut transform, mut animation_timer, mut animation_indices, mut player_state) in query.iter_mut() {
        transform.translation.y += 200. * time.delta_seconds();
        animation_indices.first = 6;
        animation_indices.last = 11;
        *player_state = PlayerState::Jumping;
        animation_timer.0.reset();
        animation_timer.0.unpause();    }
}