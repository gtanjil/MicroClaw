//! Krill Mesh Protocol (KMP) Definitions

// Status codes for network broadcasting
pub enum Status {
    Idle,
    Working,
    Success,
    Error,
}

// Protocol Constants
pub const KRILL_VERSION: u8 = 1;
pub const MESH_PORT: u16 = 8888;