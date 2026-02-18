#![no_std]
#![no_main]

use panic_halt as _;

// Architecture Fix: Conditional imports based on target CPU
#[cfg(target_arch = "riscv32")]
use riscv_rt::entry;

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;

mod agent;
mod net;
mod driver;
mod protocol;

#[entry]
fn main() -> ! {
    // 1. Initialize Hardware (GPIO/Peripherals)
    let mut hardware = driver::init();

    // 2. Initialize the MicroClaw "Krill" Brain
    let mut brain = agent::Brain::new();

    loop {
        // 3. Listen for agentic signals via Mesh (UDP/KMP)
        if let Some(signal) = net::listen_for_signal() {
            
            // 4. Process instruction through local 1-bit logic
            let action = brain.decide(signal);
            
            // 5. Execute hardware change
            hardware.execute(action);
            
            // 6. Broadcast status back to the "Swarm"
            net::broadcast_status(protocol::Status::Success);
        }
    }
}