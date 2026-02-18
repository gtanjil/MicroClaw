use crate::protocol::Status;

static mut RX_BUF: [u8; 64] = [0u8; 64];

pub fn listen_for_signal() -> Option<&'static [u8]> {
    // In real hardware, this polls the Wi-Fi/LoRa peripheral
    // For this boilerplate, we return None (waiting)
    None
}

pub fn broadcast_status(_status: Status) {
    // Logic to push UDP packet to the mesh
}