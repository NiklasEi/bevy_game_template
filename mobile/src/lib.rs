use bevy::prelude::*;
use bevy_game::GamePlugin;

#[bevy_main]
fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run()
}
