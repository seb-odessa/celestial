use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin, TouchControls};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, mouse_click_system)
        .run();
}

// This system prints messages when you press or release the left mouse button:
// fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
//     if mouse_button_input.pressed(MouseButton::Left) {
//         info!("left mouse currently pressed");
//     }

//     if mouse_button_input.just_pressed(MouseButton::Left) {
//         info!("left mouse just pressed");
//     }

//     if mouse_button_input.just_released(MouseButton::Left) {
//         info!("left mouse just released");
//     }
// }

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(139.20 / 2.0)),
        material: materials.add(Color::YELLOW),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(4.4 / 2.0)),
        material: materials.add(Color::WHITE),
        // transform: Transform::from_xyz(579.0, 0.0, 0.0),
        transform: Transform::from_xyz(579.110, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(12.1036 / 2.0)),
        material: materials.add(Color::WHITE),
        // transform: Transform::from_xyz(579.0, 0.0, 0.0),
        transform: Transform::from_xyz(1083.070, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(12.7420 / 2.0)),
        material: materials.add(Color::WHITE),
        // transform: Transform::from_xyz(579.0, 0.0, 0.0),
        transform: Transform::from_xyz(1499.780, 0.0, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 20000.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
    //         ..default()
    //     },
    //     PanOrbitCamera::default(),
    // ));
}
