use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .insert_resource(FallingTimer(Timer::from_seconds(1., TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, update_score)
        .add_systems(
            FixedUpdate,
            (move_player, spawn_falling, move_falling, collision_falling),
        )
        .run();
}

const FALLING_SPEED: f32 = 0.05;
const PICKUP_DISTANCE: f32 = 1.;

#[derive(Default)]
struct Player {
    position: Vec3,
}

#[derive(Resource, Default)]
struct Game {
    score: u32,
    falling_handle: Handle<Mesh>,
    player: Player,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PlayerComponent;

#[derive(Component)]
struct Falling;

#[derive(Resource)]
struct FallingTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1., 12.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Score text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Score: ", TextStyle::default()),
            TextSection::new("0", TextStyle::default()),
        ]),
        ScoreText,
    ));

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1., 1., 1.)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PlayerComponent,
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 3.0, 1.0),
        ..default()
    });

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10., 10.)),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0., -1., 0.),
        ..default()
    });

    // Falling object
    game.falling_handle = meshes.add(Sphere::new(0.25));
}

fn update_score(game: Res<Game>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", game.score);
}

fn move_player(
    mut game: ResMut<Game>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerComponent>>,
) {
    let mut transform = query.single_mut();
    let mut direction = 0.;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.;
    }
    const PLAYER_SPEED: f32 = 0.05;
    transform.translation.x += direction * PLAYER_SPEED;

    game.player.position = transform.translation;
}

fn spawn_falling(
    game: Res<Game>,
    time: Res<Time>,
    mut commands: Commands,
    mut timer: ResMut<FallingTimer>,
) {
    // Increment the timer, if not finished, dont do anything.
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    commands.spawn((
        PbrBundle {
            mesh: game.falling_handle.clone(),
            transform: Transform::from_xyz(0.0, 5., 0.),
            ..default()
        },
        Falling,
    ));
}

fn move_falling(mut query: Query<&mut Transform, With<Falling>>) {
    for mut tranform in query.iter_mut() {
        // Move
        tranform.translation.y -= FALLING_SPEED;
    }
}

fn collision_falling(
    mut game: ResMut<Game>,
    mut commands: Commands,
    query: Query<(&mut Transform, Entity), With<Falling>>,
) {
    for (transform, entity) in query.iter() {
        // Check for collision with player
        if transform.translation.distance_squared(game.player.position)
            < PICKUP_DISTANCE * PICKUP_DISTANCE
        {
            game.score += 1;

            commands.entity(entity).despawn();
        }
    }
}
