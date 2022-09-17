use bevy::ecs::change_detection::Mut;
use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::text::Text;

use crate::ui::counter::store::CounterStore;

/// Marker for the text entity
#[derive(Component)]
pub struct CounterStateRenderText;

/// Update text value according to Store
pub fn render_todo_text(
    state: Res<CounterStore>,
    mut render_text: Query<&mut Text, With<CounterStateRenderText>>,
) {
    render_text.get_single_mut().and_then(|mut text| { text.sections[0].value = format!("Value: {}", state.0); Ok(())});
}
