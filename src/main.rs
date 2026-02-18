#![no_std]
#![no_main]

use panic_abort as _; // Minimizes binary size
use cortex_m_rt::entry;
use heapless::String;

mod agent;
mod net;
mod driver;
mod protocol;

#[entry]
fn main() -> ! {
    // 1. Initialize Hardware (GPIO/WiFi)
    let mut hardware = driver::init();

    // 2. Initialize the "Krill" Agent
    let mut krill_agent = agent::Brain::new();

    // 3. The Main Agentic Loop (Zero-Latency)
    loop {
        // Listen for "Swarm" messages via Krill Mesh Protocol
        if let Some(instruction) = net::listen_for_signal() {
            
            // Local processing or API Proxying
            let response = krill_agent.process(instruction);
            
            // Execute physical action (e.g., flip a switch, move a motor)
            hardware.execute(response);
            
            // Broadcast status to the "Claw" mesh
            net::broadcast_status("Task Complete");
        }
    }
}