use bevy::app::{App, Plugin};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::change_detection::{Mut, ResMut};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::schedule::State;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ecs::world::World;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::transform::components::Transform;

use crate::core::level_manager::{ExitLevel, LevelManagerPlugin, LevelName};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system(handle_input)
            .add_plugin(LevelManagerPlugin);
    }
}

/// spawn a camera into the world (and implicitly possess it)
fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}

/// DEBUG - Invoke the level switching behaviors manually.
fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut exit_level: EventWriter<ExitLevel>
) {
    if keys.just_pressed(KeyCode::F1) {
        exit_level.send(ExitLevel(LevelName::SimpleScene1));
    } else if keys.just_pressed(KeyCode::F2) {
        exit_level.send(ExitLevel(LevelName::SimpleScene2));
    } else if keys.just_pressed(KeyCode::F3) {
        exit_level.send(ExitLevel(LevelName::MainMenu));
    }
}
