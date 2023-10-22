use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_ui_3d::{Interaction3d, Ui3dElementBundle, Ui3dPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Scene".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Ui3dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            ..default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube that is clickable
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Ui3dElementBundle {
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            ..default()
        },
    ));

    // cube that is not clickable
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn ui_system(interactions3d: Query<&Interaction3d, Changed<Interaction3d>>) {
    for interaction in interactions3d.iter() {
        println!("{:?}", interaction);
    }
}
