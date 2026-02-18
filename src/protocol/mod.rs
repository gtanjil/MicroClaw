#[repr(u8)]
pub enum Status {
    Success = 0,
    Failure = 1,
    Busy = 2,
}

pub const KRILL_VERSION: u8 = 1;
pub const MESH_PORT: u16 = 8888;