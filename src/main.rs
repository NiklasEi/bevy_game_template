// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_game::GamePlugin;

fn main() {
    App::new()
        // Initialize the standard Bevy engine's default features
        .add_plugins(DefaultPlugins)

        // Initialize our app's logic
        .add_plugin(GamePlugin)

        // Run the application
        .run();
}
