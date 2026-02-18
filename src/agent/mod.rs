//! The Decision Engine

// 1. Define Action PUBLICLY so Driver can see it
#[derive(Debug, Clone, Copy)]
pub enum Action {
    ToggleLight,
    PulseMotor(u32), // Run for X ms
    OffloadTask,     // Send to OpenClaw
    Standby,
}

pub struct Brain {
    pub id: u8,
}

impl Brain {
    pub fn new() -> Self {
        Self { id: 0x01 }
    }

    // 2. Fixed naming: standardized on 'decide'
    pub fn decide(&self, input: Option<&[u8]>) -> Action {
        match input {
            Some(data) => {
                // Simple byte-level pattern matching (No heavy parsing)
                match data.get(0) {
                    Some(b'1') => Action::ToggleLight,
                    Some(b'2') => Action::PulseMotor(100),
                    Some(b'9') => Action::OffloadTask,
                    _ => Action::Standby,
                }
            }
            None => Action::Standby,
        }
    }
}