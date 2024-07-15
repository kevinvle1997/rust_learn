use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .init_resource::<Gravity>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_sprite,
                apply_gravity,
                handle_player_ground_collision,
                jump,
                movement,
            )
                .chain(),
        )
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

impl PlayerState {
    fn animation_type(&self) -> AnimationType {
        match self {
            PlayerState::Running => AnimationType::Repeat,
            PlayerState::Neutral => AnimationType::Single,
            PlayerState::Jumping => AnimationType::Single,
        }
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
struct AnimationTimer(Timer);

#[derive(Component)]
enum AnimationType {
    Single,
    Repeat,
}

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct Gravity(Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(Vec2::new(0.0, -9.8 * 50.0)) // Adjust this value to change gravity strength
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &PlayerState,
    )>,
) {
    for (indices, mut timer, mut atlas, player_state) in &mut query {
        timer.tick(time.delta());
        if atlas.index < indices.first {
            atlas.index = indices.first
        }
        if timer.just_finished() {
            match player_state.animation_type() {
                AnimationType::Repeat => {
                    atlas.index = if atlas.index >= indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
                AnimationType::Single => {
                    if atlas.index < indices.last {
                        atlas.index += 1;
                    }
                }
            }
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
        Velocity::default(),
        Player,
    ));
}

fn movement(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut PlayerState,
        ),
        With<Player>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut animation_timer, mut animation_indices, mut player_state) in
        query.iter_mut()
    {
        println!("{:?}", animation_timer.0);
        if *player_state != PlayerState::Jumping {
            if keyboard_input.pressed(KeyCode::KeyA) {
                *player_state = PlayerState::Running;
                animation_indices.first = 0;
                animation_indices.last = 5;
                transform.translation.x -= 150. * time.delta_seconds();
                transform.scale.x = -6.0; // Flip the sprite horizontally
                animation_timer.0.unpause();
            } else if keyboard_input.pressed(KeyCode::KeyD) {
                *player_state = PlayerState::Running;
                animation_indices.first = 0;
                animation_indices.last = 5;
                transform.translation.x += 150. * time.delta_seconds();
                transform.scale.x = 6.0; // Flip the sprite horizontally
                animation_timer.0.unpause();
            } else {
                *player_state = PlayerState::Neutral;
                animation_timer.0.pause();
            }
        } else {
            animation_timer.0.unpause();
        }
    }
}

fn jump(
    mut query: Query<(
        &mut Velocity,
        &mut AnimationIndices,
        &mut PlayerState,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (
        mut velocity,
        mut animation_indices,
        mut player_state,
        mut animation_timer,
        mut texture_atlas,
    ) in query.iter_mut()
    {
        if keyboard_input.just_pressed(KeyCode::Space) && *player_state != PlayerState::Jumping {
            animation_timer.0.reset();
            animation_timer.0.unpause();
            texture_atlas.index = 6;
            *player_state = PlayerState::Jumping;
            velocity.0.y = 300.0; // Adjust this value to change jump strength
            animation_indices.first = 6;
            animation_indices.last = 11;
        }
    }
}

fn apply_gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Apply gravity to velocity
        velocity.0 += gravity.0 * time.delta_seconds();

        // Update position based on velocity
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}

fn handle_player_ground_collision(
    mut query: Query<(&mut Transform, &mut Velocity, &mut PlayerState), With<Player>>,
) {
    const GROUND_LEVEL: f32 = 0.0; // Adjust this if your ground is not at y = 0

    for (mut transform, mut velocity, mut player_state) in query.iter_mut() {
        // Check for ground collision
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
            *player_state = PlayerState::Neutral;
        }
    }
}
