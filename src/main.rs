use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_score)
        .run();
}

#[derive(Component, Default)]
struct Score(u32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1., 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Score text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle::default()
            ),
            TextSection::new(
                "0",
                TextStyle::default()
            ),
        ]),
        Score(0)
    ));

    // Player
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 3.0, 1.0),
        ..default()
    });
}

fn update_score(
    mut query: Query<(&mut Text, &mut Score)>
) {
    let (mut text, score) = query.single_mut();
    text.sections[1].value = format!("{}", score.0);
}