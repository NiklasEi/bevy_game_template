mod default;
mod ui;
mod maps;

use bevy::app::{App, Plugin};
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::default::state::GameState;
use crate::ui::gameplay::plugin::UIGameplayPlugin;
use crate::maps::simple_scene::SimpleScene;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the "default" GameState
            .add_state(GameState::Loading)

            // Load the default level: a simple scene
            .add_plugin(SimpleScene)

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
