use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::change_detection::{Mut, ResMut};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Query, Res};
use bevy::hierarchy::BuildChildren;
use bevy::input::Input;
use bevy::log::info;
use bevy::render::color::Color;
use bevy::text::{Text, TextAlignment, TextSection, TextStyle};
use bevy::ui::{Interaction, Size, Style, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};
use bevy::ui::widget::Button;

use crate::ui::counter::actions::{CounterActionDecrement, CounterActionIncrement};
use crate::ui::counter::facade::{handle_interaction_decrement, handle_interaction_increment};
use crate::ui::counter::reducer::{reduce_decrement, reduce_increment};
use crate::ui::counter::store::CounterStore;
use crate::ui::counter::template::render_todo_text;
use crate::ui::counter::ui::init_ui;

pub struct CounterUIPlugin;

impl Plugin for CounterUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // (Store) the State of this UI widget
            .insert_resource(CounterStore(0))

            // (Actions) Declare Actions
            .add_event::<CounterActionIncrement>()
            .add_event::<CounterActionDecrement>()

            // (Facade) Map interaction to Action
            .add_system(handle_interaction_increment)
            .add_system(handle_interaction_decrement)

            // (Reducers) Reduce Actions to changes in the Store
            .add_system(reduce_increment)
            .add_system(reduce_decrement)

            // (Template) Render front-end variables
            .add_system(render_todo_text)

            // Initialize UI
            .add_startup_system(init_ui);
    }
}
