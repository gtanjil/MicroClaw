//! Hardware Abstraction Layer

// 3. Import Action from the agent module
use crate::agent::Action;

pub struct Hardware;

pub fn init() -> Hardware {
    // In real code: Initialize GPIO pins here
    Hardware
}

impl Hardware {
    // 4. Implement execute using the imported Action enum
    pub fn execute(&mut self, action: Action) {
        match action {
            Action::ToggleLight => {
                // GPIO High/Low logic would go here
            },
            Action::PulseMotor(_ms) => {
                // PWM logic would go here
            },
            Action::OffloadTask => {
                // Blink LED to show offloading status
            },
            Action::Standby => {
                // Low power mode
            }
        }
    }
}