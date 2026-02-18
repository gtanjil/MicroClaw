pub struct Hardware {
    // Pins would go here (e.g., GPIO)
}

impl Hardware {
    pub fn init() -> Self {
        // Initialize GPIO pins here
        Self {}
    }

    pub fn apply(&mut self, command_val: i32) {
        if command_val == 1 {
            // Logic to turn Pin High (Physical Action)
        } else {
            // Logic to turn Pin Low
        }
    }
}