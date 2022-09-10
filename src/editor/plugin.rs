use bevy::app::{Plugin, App};
use crate::editor::editor::create_editor_window;
use crate::editor::simple_scene::create_simple_scene;
use crate::editor::interaction::handle_ui_interaction;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_editor_window)
            .add_startup_system(create_simple_scene)
            .add_system(handle_ui_interaction)
        ;
    }
}
