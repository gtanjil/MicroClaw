use serde::Deserialize;
use heapless::String;

#[derive(Deserialize)]
pub struct Instruction<'a> {
    pub task: &'a str,
    pub val: i32,
}

pub enum Action {
    Execute(i32), // Direct hardware control
    Offload,      // Task too complex, send to OpenClaw
    Ignore,
}

pub struct Brain {
    pub agent_id: u8,
}

impl Brain {
    pub fn new() -> Self {
        Self { agent_id: 0x01 }
    }

    pub fn decide(&mut self, payload: &[u8]) -> Action {
        // Zero-allocation JSON parsing
        match serde_json_core::from_slice::<Instruction>(payload) {
            Ok((instruction, _)) => {
                match instruction.task {
                    "light" | "relay" | "actuate" => Action::Execute(instruction.val),
                    "think" | "search" | "code" => Action::Offload,
                    _ => Action::Ignore,
                }
            }
            Err(_) => Action::Ignore,
        }
    }
}