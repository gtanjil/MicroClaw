pub struct Brain {
    pub id: u8,
}

pub enum Action {
    Pulse(u32),
    Toggle,
    Offload,
    None,
}

impl Brain {
    pub fn new() -> Self {
        Self { id: 0x01 }
    }

    pub fn decide(&self, input: &[u8]) -> Action {
        // Very basic pattern matching for speed/size
        if input.is_empty() { return Action::None; }
        
        match input[0] {
            b'1' => Action::Toggle,
            b'2' => Action::Pulse(100),
            b'9' => Action::Offload, // Send to OpenClaw
            _ => Action::None,
        }
    }
}