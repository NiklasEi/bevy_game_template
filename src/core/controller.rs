use bevy::app::{App, Plugin};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::change_detection::{Mut, ResMut};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ecs::world::World;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::transform::components::Transform;

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
    }
}


/// LEVEL MANAGER
pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnterLevel>()
            .add_event::<ExitLevel>()
            .add_system(unload_level);
    }
}

/// Attach this label to everything associated with a given level, so that they
///     can be unloaded when we exit the level and switch to a new one.
#[derive(Component)]
pub struct UnloadOnLevelChange;

/// Broadcast this event when you want to load a level! Its Plugin will listen
///     for its LevelName and load accordingly!
///
/// NOTE | The "LevelName" for both events below refer to the new level to which
///     we now want to travel!
pub struct EnterLevel(pub LevelName);
pub struct ExitLevel(pub LevelName);

/// Singleton reference object so that we know which level's
///     assets should be loaded for the player at any given moment.
struct LevelManager {
    current_level: LevelName,
}

#[derive(Clone, Copy, Debug)]
pub enum LevelName {
    SimpleScene1,
    SimpleScene2
}

/// Delete all of the existing entities from the current level!
fn unload_level(
    mut exit_level: EventReader<ExitLevel>,
    mut enter_level: EventWriter<EnterLevel>,
    mut entities: Query<Entity, With<UnloadOnLevelChange>>,
    mut commands: Commands) {
    exit_level.iter().for_each(|it| {
        warn!("Exiting level...");
        let it: &ExitLevel = it;

        // Unload all relevant entities
        entities.iter_mut().for_each(|e| {
            let mut e: Entity = e;
            commands.entity(e).despawn_recursive();
        });

        // Load the relevant next level!
        enter_level.send(EnterLevel(it.0));
    });
}
