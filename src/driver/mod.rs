use crate::agent::Action;

pub struct HardwareHandler;

pub fn init() -> HardwareHandler {
    // Configure GPIO Pins here
    HardwareHandler
}

impl HardwareHandler {
    pub fn execute(&mut self, action: Action) {
        match action {
            Action::Toggle => { /* Toggle Pin */ },
            Action::Pulse(ms) => { /* PWM Pulse for ms */ },
            Action::Offload => { /* Do nothing, handled by net */ },
            Action::None => {},
        }
    }
}