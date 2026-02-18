#![no_std]
#![no_main]

// Panic handler is required for no_std
use panic_halt as _;

// FIXED: Using riscv_rt (RISC-V Runtime) instead of cortex_m_rt
use riscv_rt::entry;

// Import our local modules
mod agent;
mod net;
mod driver;
mod protocol;

#[entry]
fn main() -> ! {
    // 1. Initialize Hardware Logic
    // In a real scenario, this sets up GPIO, WiFi, and Peripherals
    let mut hardware = driver::init();
    
    // 2. Initialize the "Krill" Brain
    let brain = agent::Brain::new();

    // 3. The Agentic Loop (Infinite)
    loop {
        // A. SENSE: Poll the network for Swarm instructions
        // This returns Option<&[u8]>
        let signal = net::listen_for_signal();

        // B. THINK: The Brain decides what to do based on the signal
        // We use .decide() as defined in agent/mod.rs
        let action = brain.decide(signal);

        // C. ACT: Execute the physical action
        // We pass the Action enum (ToggleLight, PulseMotor, etc.)
        hardware.execute(action);

        // D. COMMUNICATE: Update the Swarm
        // If we did something, tell the mesh network
        match action {
            agent::Action::Standby => {
                // Do nothing, save power
            },
            _ => {
                // Broadcast success to the Claw ecosystem
                net::broadcast_status(protocol::Status::Success);
            }
        }
    }
}