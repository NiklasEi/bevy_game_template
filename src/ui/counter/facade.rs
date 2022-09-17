use bevy::ecs::event::EventWriter;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::Query;
use bevy::ui::Interaction;

use crate::ui::counter::actions::{CounterActionDecrement, CounterActionIncrement};

/// Map button interactions to Increment Actions for updating UI State
pub fn handle_interaction_increment(
    mut button: Query<
        &Interaction,
        (With<CounterActionIncrement>, Changed<Interaction>)
    >,
    mut action: EventWriter<CounterActionIncrement>
) {
    button.get_single().iter()
        .filter(|it| Interaction::Clicked.eq(it))
        .for_each(|_| action.send(CounterActionIncrement));
}

/// Map button interactions to Dencrement Actions for updating UI State
pub fn handle_interaction_decrement(
    mut button: Query<
        &Interaction,
        (With<CounterActionDecrement>, Changed<Interaction>)
    >,
    mut action: EventWriter<CounterActionDecrement>
) {
    button.get_single().iter()
        .filter(|it| Interaction::Clicked.eq(it))
        .for_each(|it| action.send(CounterActionDecrement));
}