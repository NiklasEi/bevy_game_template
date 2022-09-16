use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, ResMut};
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::render::color::Color;
use bevy::render::view::{ComputedVisibility, Visibility};
use bevy::ui::{Size, Style, Val};
use bevy::ui::entity::NodeBundle;

use crate::ui::style::get_standard_padding;

pub struct UIMainMenuPlugin;

impl Plugin for UIMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UIShowMainMenuEvent>()
            .add_startup_system(init_main_menu);
    }
}

/// Broadcast this event struct whenever you want to render the Main Menu UI onto the screen.
pub struct UIShowMainMenuEvent;

#[derive(Component)]
pub struct MainMenuLabel;

/// Initialize the main menu assets
fn init_main_menu(mut commands: Commands) {
    commands.spawn_bundle(build_parent())
        .with_children(build_standard_panel())
        .with_children(build_standard_panel())
        .with_children(build_standard_panel())
        .insert(MainMenuLabel);
}

pub fn build_standard_panel() -> fn(&mut ChildBuilder) {
    |builder: &mut ChildBuilder| {
        builder.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(98.0), Val::Px(60.0)),
                margin: get_standard_padding(),
                ..Default::default()
            },
            color: Color::DARK_GREEN.into(),
            ..Default::default()
        });
    }
}

/// Create the parent MainMenu UI widget
fn build_parent() -> NodeBundle {
    NodeBundle {
        computed_visibility: ComputedVisibility::not_visible(),
        ..Default::default()
    }
}
