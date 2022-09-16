use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::ecs::change_detection::ResMut;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::prelude::Res;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Query};
use bevy::hierarchy::BuildChildren;
use bevy::input::Input;
use bevy::log::info;
use bevy::prelude::{KeyCode, NodeBundle};
use bevy::render::color::Color;
use bevy::text::{Text, TextAlignment, TextSection, TextStyle};
use bevy::ui::{Interaction, Size, Style, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, TextBundle};
use bevy::ui::widget::Button;

pub struct TodoUIPlugin;

impl Plugin for TodoUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TodoState(0))
            .add_event::<TodoAction>()
            .add_system(handle_todo_action)
            .add_system(map_todo_action)
            .add_startup_system(init_todoui);
    }
}

/// Singleton: stores the current state
struct TodoState(pub i32);

/// Serve both as a Marker Component (for UI buttons) and as an Event for mutating state.
#[derive(Component)]
enum TodoAction {
    Increment,
    Decrement
}

/// Marker for the increment button
#[derive(Component)]
struct TodoActionIncrement;

/// Marker for the decrement button
#[derive(Component)]
struct TodoActionDecrement;

/// Marker for the text entity
#[derive(Component)]
struct TodoActionStateText;

fn handle_todo_action(
    mut todo_action: EventReader<TodoAction>,
    mut todo_state: ResMut<TodoState>
) {
    todo_action.iter().for_each(|it| {
        match it {
            TodoAction::Increment => {
                todo_state.0 = todo_state.0 + 1;
            },
            TodoAction::Decrement => {
                todo_state.0 = todo_state.0 - 1;
            },
            _ => {}
        }

        info!("todo_state={}", todo_state.0)
    })
}

/// TODO | This should be handling UI clicks on buttons, rather than reading from keyboard input.
fn map_todo_action(
    input: Res<Input<KeyCode>>,
    mut todo_action: EventWriter<TodoAction>,
    todo_state: Res<TodoState>,
    mut todo_state_text: Query<&mut Text, With<TodoActionStateText>>,
    mut increment_action: Query<&Interaction, (With<Button>, With<TodoActionIncrement>, Changed<Interaction>)>,
    mut decrement_action: Query<&Interaction, (With<Button>, With<TodoActionDecrement>, Changed<Interaction>)>
) {
    for mut text in &mut todo_state_text {
        let text: &mut Text = &mut text;
        text.sections[0].value = format!("Value: {}", todo_state.0);
    }

    input.get_just_pressed().for_each(|it| {
        match it {
            KeyCode::F5 => { todo_action.send(TodoAction::Increment); },
            KeyCode::F6 => { todo_action.send(TodoAction::Decrement); },
            _ => {}
        }
    });
    increment_action.iter().for_each(|it| {
        let it: &Interaction = it;
        match it {
            Interaction::Clicked => { todo_action.send(TodoAction::Increment); },
            _ => {}
        }
    });
    decrement_action.iter().for_each(|it| {
        let it: &Interaction = it;
        match it {
            Interaction::Clicked => { todo_action.send(TodoAction::Decrement); },
            _ => {}
        }
    });
}

fn init_todoui(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>
) {
    commands
        .spawn_bundle(NodeBundle {
            color: Color::hsla(0.1,0.1,0.1,0.3).into(),
            style: Style {
                size: Size::new(Val::Px(3.0*98.0), Val::Px(3.0*60.0)),
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
                "PLACEHOLDER",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                }
            )
                .with_text_alignment(TextAlignment::TOP_CENTER))
                .insert(TodoActionStateText);
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
                .insert(TodoActionIncrement);
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
                .insert(TodoActionDecrement);
        });
}