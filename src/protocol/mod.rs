//! Krill Mesh Protocol (KMP)
//! A lightweight, binary-first protocol for agentic swarms.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)] 
pub struct KrillPacket {
    pub magic_byte: u8,    // Always 0xAA for MicroClaw
    pub version: u8,       // Protocol version
    pub sender_id: u16,    // Unique ID of the Krill agent
    pub command: u8,       // 0: Task, 1: Result, 2: Offload
    pub payload_len: u8,
}

pub const KMP_HEADER: u8 = 0xAA;

impl KrillPacket {
    pub fn new(sender: u16, cmd: u8) -> Self {
        Self {
            magic_byte: KMP_HEADER,
            version: 1,
            sender_id: sender,
            command: cmd,
            payload_len: 0,
        }
    }

    /// Serializes the packet into a byte buffer for UDP broadcast
    pub fn serialize(&self) -> [u8; 6] {
        [
            self.magic_byte,
            self.version,
            (self.sender_id >> 8) as u8,
            (self.sender_id & 0xFF) as u8,
            self.command,
            self.payload_len,
        ]
    }
}