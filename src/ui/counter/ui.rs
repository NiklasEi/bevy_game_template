use bevy::asset::AssetServer;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, ResMut};
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::{TextAlignment, TextStyle};
use bevy::ui::{Size, Style, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};

use crate::ui::counter::actions::{CounterActionDecrement, CounterActionIncrement};
use crate::ui::counter::template::CounterStateRenderText;

pub fn init_ui(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>
) {
    CounterUI{}.init(commands, asset_server);
}

struct CounterUI;

impl CounterUI {
    /// Initialize the CounterUI
    pub fn init(&self, mut commands: Commands, mut server: ResMut<AssetServer>) {
        commands
            .spawn_bundle(NodeBundle {
                color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
                style: Style {
                    size: Size::new(Val::Px(3.0 * 98.0), Val::Px(3.0 * 60.0)),
                    margin: UiRect::new(
                        Val::Px(6.0),
                        Val::Px(6.0),
                        Val::Px(2.0),
                        Val::Px(2.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|builder| {
                builder.spawn_bundle(TextBundle::from_section(
                    "DEFAULT TEXT",
                    TextStyle {
                        font: server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    }
                )
                    .with_text_alignment(TextAlignment::TOP_CENTER))
                    .insert(CounterStateRenderText);
            })
            .with_children(|builder| {
                builder.spawn_bundle(ButtonBundle {
                    color: Color::GREEN.into(),
                    style: Style {
                        size: Size::new(Val::Px(98.0), Val::Px(60.0)),
                        margin: UiRect::new(
                            Val::Px(6.0),
                            Val::Px(6.0),
                            Val::Px(2.0),
                            Val::Px(2.0),
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(CounterActionIncrement);
                builder.spawn_bundle(ButtonBundle {
                    color: Color::RED.into(),
                    style: Style {
                        size: Size::new(Val::Px(98.0), Val::Px(60.0)),
                        margin: UiRect::new(
                            Val::Px(6.0),
                            Val::Px(6.0),
                            Val::Px(2.0),
                            Val::Px(2.0),
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(CounterActionDecrement);
            });
    }
}
