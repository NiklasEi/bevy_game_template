use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::schedule::{State, SystemSet};
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::ecs::world::World;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::warn;
use bevy::reflect::TypeRegistry;
use bevy::scene::{DynamicScene, DynamicSceneBundle};

/// LEVEL MANAGER
pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnterLevel>()
            .add_event::<ExitLevel>()
            .add_system(unload_level)
            .add_system(load_level);
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

/// Enum for each level of the game.
///
/// TODO | Promote this to some kind of asset file, so that it's not hard-coded.
#[derive(Clone, Copy, Debug)]
pub enum LevelName {
    // Default (no world)
    MainMenu,

    // maps/simple_scene.rs
    SimpleScene1,

    // maps/simple_scene2.rs
    SimpleScene2
}

/// Delete all of the existing entities from the current level!
fn unload_level(
    mut exit_level: EventReader<ExitLevel>,
    mut enter_level: EventWriter<EnterLevel>,
    mut entities: Query<Entity, With<UnloadOnLevelChange>>,
    mut commands: Commands
) {
    exit_level.iter().for_each(|it| {
        warn!("Exiting level...");
        let it: &ExitLevel = it;

        // Delete all entities in the current world
        entities.iter_mut().for_each(|e| {
            let e: Entity = e;
            commands.entity(e).despawn_recursive();
        });

        enter_level.send(EnterLevel(it.0));
    });
}

/// Initialize all of the assets for the given level
fn load_level(
    mut enter_level: EventReader<EnterLevel>
) {
    enter_level.iter().for_each(|it| {
        // NOTE | Currently, each maps/*.rs file uses its own event listener
        //  Maybe I should use some kind of trait or resource?
        //  I should do discovery work on the engine's DynamicSceneBundle.
    });
}