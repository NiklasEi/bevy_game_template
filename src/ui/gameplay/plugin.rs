use bevy::app::{App, Plugin};

use crate::ui::gameplay::interaction::handle_ui_interaction;
use crate::ui::gameplay::widgets::init_gameplay_ui;

pub struct UIGameplayPlugin;

impl Plugin for UIGameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_gameplay_ui)

            // UI interactions should be monitored
            .add_system(handle_ui_interaction)
        ;
    }
}