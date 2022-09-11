use bevy::log::warn;
use bevy::ecs::system::Commands;
use bevy::ui::entity::{NodeBundle, ButtonBundle};
use bevy::render::color::Color;
use bevy::ui::{Style, Val, Size, PositionType, UiRect, FlexDirection, FocusPolicy, AlignSelf};
use bevy::transform::components::Transform;
use bevy::math::Vec3;
use bevy::hierarchy::{BuildChildren, ChildBuilder};

use crate::ui::button::get_standard_button;
use crate::ui::style::{get_standard_padding, TRANSPARENT_BLACK};

/// Spawn the Gameplay UI
pub fn init_gameplay_ui(mut commands: Commands) {
    commands
        .spawn_bundle(get_fullscreen_canvas())
        .with_children(build_topleft_ui())
        .with_children(build_topright_ui())
        .with_children(build_bottom_ui());
}

fn build_topleft_ui() -> fn(&mut ChildBuilder) {
    |builder: &mut ChildBuilder| {
        builder.spawn_bundle(get_standard_panel());
        builder.spawn_bundle(get_standard_button());
    }
}

/// Parent for the "Resources Bar" at the top right of the screen
fn build_topright_ui() -> fn(&mut ChildBuilder) {
    |builder: &mut ChildBuilder| {
        builder.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(5.0)),
                position: UiRect::new(
                    Val::Percent(70.0),  // 100.0 - size.width
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(95.0)  // 100.0 - size.height
                ),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: TRANSPARENT_BLACK.into(),
            ..Default::default()
        });
    }
}

/// Parent for the RTS "Command Bar" at the bottom of the screen
fn build_bottom_ui() -> fn(&mut ChildBuilder) {
    |builder: &mut ChildBuilder| {
        builder.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                position: UiRect::new(
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(0.0)
                ),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: TRANSPARENT_BLACK.into(),
            ..Default::default()
        });
    }
}

pub fn get_standard_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(98.0), Val::Px(60.0)),
            margin: get_standard_padding(),
            ..Default::default()
        },
        color: Color::hsl(rand::random::<f32>() * 360.0, 0.68, 0.68).into(),
        ..Default::default()
    }
}

pub fn get_fullscreen_canvas() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: Color::NONE.into(),
        focus_policy: FocusPolicy::Pass,
        ..Default::default()
    }
}
