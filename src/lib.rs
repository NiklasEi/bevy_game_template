use bevy::app::{App, Plugin};
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::core::controller::PlayerControllerPlugin;
use crate::default::state::GameState;
use crate::maps::simple_scene2::SimpleScene2;
use crate::maps::simple_scene::SimpleScene;
use crate::ui::gameplay::plugin::UIGameplayPlugin;

mod default;  // provided by the starter template

mod core;
mod ui;
mod maps;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the "default" GameState
            .add_state(GameState::Loading)

            // Init the level asset loaders
            .add_plugin(SimpleScene)
            .add_plugin(SimpleScene2)

            // Spawn the player (i.e. camera) in the map
            .add_plugin(PlayerControllerPlugin)

            // Init and render the Gameplay UI
            .add_plugin(UIGameplayPlugin)

            // Exit the application when the Escape button is pressed
            .add_system(bevy::window::close_on_esc);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
