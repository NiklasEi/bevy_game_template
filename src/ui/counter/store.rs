/// Singleton: Store for the Counter UI
pub struct CounterStore {

    pub value: i32

}

impl CounterStore {

    /// Create a new Store instance with the given initial value
    pub fn new(initial_value: i32) -> Self {
        CounterStore {
            value: initial_value
        }
    }

    /// Mutate the `value` by the given amount
    pub fn increment(&mut self, amount: i32) {
        self.value = self.value + amount;
    }

    /// Reference the current underlying `value`
    pub fn get_value(&self) -> i32 {
        self.value
    }

}
