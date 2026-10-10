use crate::loading::TextureAssets;
use crate::{GameResult, GameState};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                update_button_colors.run_if(in_state(GameState::Menu)),
            )
            .add_observer(activate_button)
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    game_result: Option<Res<GameResult>>,
) {
    let last_result = game_result.map(|result| match *result {
        GameResult::Won => ("You won!", Color::hsl(120., 0.95, 0.7)),
        GameResult::Lost => ("You lost!", Color::hsl(0., 0.95, 0.7)),
    });
    commands.spawn((Camera2d, Msaa::Off, Menu));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            if let Some((text, color)) = last_result {
                children.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: FontSize::Px(60.0),
                        ..default()
                    },
                    TextColor(color),
                    Node {
                        margin: UiRect::bottom(Val::Px(30.)),
                        ..default()
                    },
                ));
            }
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    Hovered::default(),
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font_size: FontSize::Px(40.0),
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Hovered::default(),
                    Node {
                        width: Val::Px(180.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: FontSize::Px(15.0),
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Button,
                    Hovered::default(),
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: FontSize::Px(15.0),
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn update_button_colors(
    mut buttons: Query<(&Hovered, &mut BackgroundColor, &ButtonColors), Changed<Hovered>>,
) {
    for (hovered, mut color, button_colors) in &mut buttons {
        *color = if hovered.get() {
            button_colors.hovered
        } else {
            button_colors.normal
        }
        .into();
    }
}

fn activate_button(
    activate: On<Activate>,
    mut next_state: ResMut<NextState<GameState>>,
    buttons: Query<(Option<&ChangeState>, Option<&OpenLink>)>,
) {
    let Ok((change_state, open_link)) = buttons.get(activate.entity) else {
        return;
    };
    if let Some(state) = change_state {
        next_state.set(state.0.clone());
    } else if let Some(link) = open_link
        && let Err(error) = webbrowser::open(link.0)
    {
        warn!("Failed to open link {error:?}");
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
