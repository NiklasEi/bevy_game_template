use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;

use crate::ui::counter::actions::{CounterActionDecrement, CounterActionIncrement};
use crate::ui::counter::store::CounterStore;

/// Increase the State value by 1
pub fn reduce_increment(
    mut increment: EventReader<CounterActionIncrement>,
    mut state: ResMut<CounterStore>
) {
    increment.iter().for_each(|it| { state.increment(1); });
}

/// Reduce the State value by 1
pub fn reduce_decrement(
    mut decrement: EventReader<CounterActionDecrement>,
    mut state: ResMut<CounterStore>
) {
    decrement.iter().for_each(|_| { state.increment(-1); });
}
