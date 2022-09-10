use bevy::log::warn;
use bevy::ecs::system::Commands;
use bevy::ui::entity::{NodeBundle, ButtonBundle};
use bevy::render::color::Color;
use bevy::ui::{Style, Val, Size, PositionType, UiRect, FlexDirection, FocusPolicy};
use bevy::transform::components::Transform;
use bevy::math::Vec3;
use bevy::hierarchy::{BuildChildren, ChildBuilder};

/// Spawn the Editor window
pub fn create_editor_window(mut commands: Commands) {
    let build_topleft_ui = |builder: &mut ChildBuilder| {
        builder.spawn_bundle(get_panel());
        builder.spawn_bundle(get_panel());
        builder.spawn_bundle(get_panel());
        builder.spawn_bundle(get_button());
        builder.spawn_bundle(get_button());
    };

    let build_topright_ui = |builder: &mut ChildBuilder| {
        builder.spawn_bundle(get_panel());
        builder.spawn_bundle(get_panel());
        builder.spawn_bundle(get_button());
        builder.spawn_bundle(get_button());
    };

    commands
        .spawn_bundle(get_fullscreen_canvas())
        .with_children(build_topleft_ui)
        .with_children(build_topright_ui);
}

pub fn get_panel() -> NodeBundle {
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

pub fn get_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(98.0), Val::Px(60.0)),
            margin: get_standard_padding(),
            position: UiRect::new(
                Val::Percent(60.0),
                Val::Percent(0.0),
                Val::Percent(0.0),
                Val::Percent(0.0),
            ),
            ..Default::default()
        },
        color: Color::hsl(rand::random::<f32>() * 360.0, 0.68, 0.32).into(),
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

// Miscellaneous constants
fn get_standard_padding() -> UiRect<Val> {
    UiRect::new(
        Val::Px(6.0),
        Val::Px(6.0),
        Val::Px(2.0),
        Val::Px(2.0),
    )
}