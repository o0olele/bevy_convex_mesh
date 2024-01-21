#![feature(array_chunks)]
use bevy::prelude::*;
use bevy_scene::convex;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    let plist: Vec<f32> = vec![
        0.,
        3.568220853805542,
        9.341723442077637,
        5.773502826690674,
        5.773502826690674,
        5.773502826690674,
        -5.773502826690674,
        5.773502826690674,
        5.773502826690674,
        3.568220853805542,
        9.341723442077637,
        0.,
        -3.568220853805542,
        9.341723442077637,
        0.,
        9.341723442077637,
        0.,
        3.568220853805542,
        9.341723442077637,
        0.,
        -3.568220853805542,
        5.773502826690674,
        5.773502826690674,
        -5.773502826690674,
        5.773502826690674,
        -5.773502826690674,
        -5.773502826690674,
        0.,
        -3.568220853805542,
        -9.341723442077637,
        0.,
        3.568220853805542,
        -9.341723442077637,
        -5.773502826690674,
        -5.773502826690674,
        -5.773502826690674,
        -9.341723442077637,
        0.,
        -3.568220853805542,
        -5.773502826690674,
        5.773502826690674,
        -5.773502826690674,
        -3.568220853805542,
        -9.341723442077637,
        0.,
        -5.773502826690674,
        -5.773502826690674,
        5.773502826690674,
        -9.341723442077637,
        0.,
        3.568220853805542,
        0.,
        -3.568220853805542,
        9.341723442077637,
        3.568220853805542,
        -9.341723442077637,
        0.,
        5.773502826690674,
        -5.773502826690674,
        5.773502826690674,
    ];
    let points: Vec<Vec3> = plist
        .array_chunks()
        .into_iter()
        .map(|&v: &[f32; 3]| Vec3::from_array(v))
        .collect();
    let mesh = convex::convex_mesh(points).unwrap();
    // convex mesh
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3 { x: 0.1, y: 0.1, z: 0.1 }),
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
        transform: Transform::from_xyz(0., 8.5, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
