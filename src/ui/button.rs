use bevy::render::color::Color;
use bevy::ui::entity::ButtonBundle;
use bevy::ui::{Style, Size, Val, UiRect};

use crate::ui::style::{NORMAL_BUTTON, get_standard_padding };

/// Provide a standard button Bundle
pub fn get_standard_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(98.0), Val::Px(60.0)),
            margin: get_standard_padding(),
            position: UiRect::new(
                Val::Percent(0.0),
                Val::Percent(0.0),
                Val::Percent(0.0),
                Val::Percent(0.0),
            ),
            ..Default::default()
        },
        color: NORMAL_BUTTON.into(),
        ..Default::default()
    }
}