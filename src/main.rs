#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

mod agent;
mod net;
mod driver;
mod protocol;

#[entry]
fn main() -> ! {
    // Initialize Hardware Peripherals
    let mut hardware = driver::init();
    
    // Initialize the Agent Brain
    let mut brain = agent::Brain::new();

    loop {
        // 1. Listen for signals from the Mesh or Local sensors
        if let Some(signal) = net::listen_for_signal() {
            
            // 2. Process logic
            let response = brain.process(signal);
            
            // 3. Act physically
            hardware.execute(response);
            
            // 4. Report back to the Swarm
            net::broadcast_status("TASK_COMPLETE");
        }
    }
}