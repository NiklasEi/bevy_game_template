use bevy::app::{App, Plugin};
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::core::controller::PlayerControllerPlugin;
use crate::maps::main_menu::MainMenuScenePlugin;
use crate::maps::simple_scene2::SimpleScene2Plugin;
use crate::maps::simple_scene::SimpleScenePlugin;
use crate::ui::counter::plugin::CounterUIPlugin;
use crate::ui::gameplay::plugin::UIGameplayPlugin;
use crate::ui::main_menu::main_menu::UIMainMenuPlugin;

mod default;  // provided by the starter template

mod core;
mod ui;
mod maps;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Init the level asset loaders
            .add_plugin(SimpleScenePlugin)
            .add_plugin(SimpleScene2Plugin)
            .add_plugin(MainMenuScenePlugin)
            .add_plugin(CounterUIPlugin)

            // Spawn the player (i.e. camera) in the map
            .add_plugin(PlayerControllerPlugin)

            // Init and render the Gameplay UI
            // .add_plugin(UIGameplayPlugin)
            // .add_plugin(UIMainMenuPlugin)

            // Exit the application when the Escape button is pressed
            .add_system(bevy::window::close_on_esc);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
