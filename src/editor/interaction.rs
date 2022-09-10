use bevy::log::warn;
use bevy::ecs::system::Query;
use bevy::ui::{FocusPolicy, Interaction, UiColor};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::{With, Changed};
use bevy::hierarchy::Children;
use bevy::ui::widget::Button;
use bevy::render::color::Color;

pub fn handle_ui_interaction(mut interaction_query: Query<
    (&Interaction, &mut UiColor),
    (Changed<Interaction>, With<Button>),
>) {
    for (state, mut color) in &mut interaction_query {
        match state {
            Interaction::Hovered => { *color = Color::GREEN.into(); },
            Interaction::Clicked => { *color = Color::RED.into(); },
            _ => {}
        }
    }
}