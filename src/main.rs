use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite,jump, movement).chain())
        .run();
}

#[derive(Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
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
        println!("{} {} {}", atlas.index, indices.first, indices.last);
        if atlas.index < indices.first {
            atlas.index = indices.first
        }
        if timer.just_finished() {
            atlas.index = if atlas.index >= indices.last {
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
        println!("{:?}", player_state);
        if *player_state == PlayerState::Neutral {
            animation_timer.0.pause();
        } else {
            animation_timer.0.unpause();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            *player_state = PlayerState::Running;
            animation_indices.first = 0;
            animation_indices.last = 5;
            transform.translation.x -= 150. * time.delta_seconds();
            transform.scale.x = -6.0; // Flip the sprite horizontally
        } if keyboard_input.pressed(KeyCode::KeyD) {
            *player_state = PlayerState::Running;
            animation_indices.first = 0;
            animation_indices.last = 5;
            transform.translation.x += 150. * time.delta_seconds();
            transform.scale.x = 6.0; // Flip the sprite horizontally
        } if !keyboard_input.pressed(KeyCode::KeyA) && !keyboard_input.pressed(KeyCode::KeyD) {
            *player_state = PlayerState::Neutral;
        }

    }
}

fn jump(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, &mut AnimationIndices, &mut PlayerState)>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for (mut transform, mut animation_indices, mut player_state) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::Space) {
        *player_state = PlayerState::Jumping;
        transform.translation.y += 200. * time.delta_seconds();
        animation_indices.first = 6;
        animation_indices.last = 11;

    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        *player_state = PlayerState::Jumping;
        transform.translation.y -= 200. * time.delta_seconds();
        animation_indices.first = 6;
        animation_indices.last = 11;

    }
    }
    
}