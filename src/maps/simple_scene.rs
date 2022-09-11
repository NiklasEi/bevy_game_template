use bevy::ecs::system::{Commands, ResMut};
use bevy::asset::Assets;
use bevy::render::mesh::{Mesh, shape};
use bevy::pbr::{StandardMaterial, PbrBundle, PointLightBundle, PointLight};
use bevy::render::color::Color;
use bevy::utils::default;
use bevy::transform::components::Transform;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::math::Vec3;
use bevy::app::{Plugin, App};

pub struct SimpleScene;

impl Plugin for SimpleScene {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_simple_scene);
    }
}

/// Set up a simple 3D scene
fn create_simple_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}