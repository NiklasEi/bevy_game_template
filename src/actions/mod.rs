use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::actions::game_control::{get_movement, GameControl};
use crate::player::Player;
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    // Todo: simplify! This also produces wrong touch positions (may be related to https://github.com/bevyengine/bevy/issues/7528)
    if let Some(touch_position) = touch_input.first_pressed_position() {
        let resolution = &window.single().resolution;
        let dimensions = Vec2::new(-resolution.width(), resolution.height());
        let diff = Vec2::new(touch_position.x, -touch_position.y)
            - player.single().translation.xy()
            + dimensions
                / (2.
                    * resolution
                        .scale_factor_override()
                        .unwrap_or(resolution.scale_factor()) as f32);
        player_movement = diff.normalize();
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
}
