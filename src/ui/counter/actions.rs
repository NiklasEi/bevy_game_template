use bevy::ecs::component::Component;

/// Marker for the increment button
#[derive(Component)]
pub struct CounterActionIncrement;

/// Marker for the decrement button
#[derive(Component)]
pub struct CounterActionDecrement;
