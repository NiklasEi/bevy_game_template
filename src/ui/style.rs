use bevy::ui::{UiRect, Val};
use bevy::render::color::Color;

pub const NORMAL_BUTTON: Color = Color::ALICE_BLUE;
pub const HOVERED_BUTTON: Color = Color::GRAY;
pub const PRESSED_BUTTON: Color = Color::DARK_GRAY;

pub const TRANSPARENT_BLACK: Color = Color::hsla(0.0, 0.0, 0.1, 0.6);


// Miscellaneous constants
pub fn get_standard_padding() -> UiRect<Val> {
    UiRect::new(
        Val::Px(6.0),
        Val::Px(6.0),
        Val::Px(2.0),
        Val::Px(2.0),
    )
}