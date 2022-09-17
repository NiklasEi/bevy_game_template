use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;

use crate::ui::counter::actions::{CounterActionDecrement, CounterActionIncrement};
use crate::ui::counter::store::CounterStore;

pub fn reduce_increment(
    mut increment: EventReader<CounterActionIncrement>,
    mut state: ResMut<CounterStore>
) {
    increment.iter().for_each(|it| { state.0 = state.0 + 1; });
}

pub fn reduce_decrement(
    mut decrement: EventReader<CounterActionDecrement>,
    mut state: ResMut<CounterStore>
) {
    decrement.iter().for_each(|_| { state.0 = state.0 - 1; });
}