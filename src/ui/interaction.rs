use bevy::ecs::system::Query;
use bevy::ui::{FocusPolicy, Interaction, UiColor};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::{With, Changed};
use bevy::hierarchy::Children;
use bevy::ui::widget::Button;
use bevy::render::color::Color;
use bevy::ecs::change_detection::Mut;

use crate::ui::style::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

pub fn handle_ui_interaction(mut interaction_query: Query<
    (&Interaction, &mut UiColor),
    (Changed<Interaction>, With<Button>),
>) {
    for (state, color) in &mut interaction_query {
        // Explicitly call out the expected types, for IntelliSense
        let state: &Interaction = state;
        let mut color: Mut<UiColor> = color;

        match state {
            Interaction::Hovered => { *color = HOVERED_BUTTON.into(); },
            Interaction::Clicked => { *color = PRESSED_BUTTON.into(); },
            Interaction::None => { *color = NORMAL_BUTTON.into(); }
        }
    }
}
