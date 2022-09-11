use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, ResMut};
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial};
use bevy::render::color::Color;
use bevy::render::mesh::{Mesh, shape};
use bevy::transform::components::Transform;
use bevy::utils::default;

use crate::core::controller::{EnterLevel, LevelName, PlayerControllerPlugin, UnloadOnLevelChange};

pub struct SimpleScene;

impl Plugin for SimpleScene {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_assets);
    }
}

/// Set up a simple 3D scene
fn spawn_assets(
    mut enter_level: EventReader<EnterLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    enter_level.iter().for_each(|it| {
        match it.0 {
            LevelName::SimpleScene1 => {
                warn!("*** Entering Simplescene1...");

                // plane
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    ..default()
                }).insert(UnloadOnLevelChange);

                // cube
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.6 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                }).insert(UnloadOnLevelChange);

                // light
                commands.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        intensity: 1500.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(4.0, 8.0, 4.0),
                    ..default()
                }).insert(UnloadOnLevelChange);
            },
            _ => {}
        }
    });
}