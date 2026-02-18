<p align="center">
  <img src="https://i.postimg.cc/Z0tX6v9N/file-000000003e707209bf553384c86fbba0.png" width="300" alt="MicroClaw Logo">
</p>

# MicroClaw (The Krill)🦐
**The World's Smallest "Bare-Metal" Agentic AI Framework.**

MicroClaw is a zero-overhead, `no_std` Rust implementation of the Claw agentic ecosystem. While **OpenClaw** handles the heavy lifting on desktops, **MicroClaw** (The Krill) lives inside the smallest sensors, switches, and edge devices, creating a decentralized "swarm" of intelligence.

---

## 📊 Technical Specifications & Comparison

MicroClaw is designed to outperform its predecessors in extreme resource-constrained environments.

| Feature | OpenClaw (Lobster) | PicoClaw (Shrimp) | **MicroClaw (Krill)** |
| :--- | :--- | :--- | :--- |
| **Language** | TypeScript / Node.js | Go | **Rust (Bare Metal)** |
| **Runtime** | Node VM (Heavy) | Go Runtime (GC) | **Zero Runtime** |
| **Binary Size** | ~200 MB | ~8 MB | **< 400 KB** |
| **RAM Usage** | ~500 MB+ | ~10 MB | **< 800 KB** |
| **Startup Time**| ~2 Seconds | ~500ms | **< 10ms** |
| **Connectivity**| WebSockets / HTTP | TCP / MQTT | **UDP Mesh (KMP)** |
| **Hardware** | Mac Mini / PC | Raspberry Pi / SBC | **$2 ESP32 / RISC-V** |

---

## 🔥 Key Features

- **Zero-Standard Library (`no_std`):** MicroClaw doesn't need Linux or Windows. It runs directly on the chip's "bare metal."
- **Krill Mesh Protocol (KMP):** A proprietary, ultra-lightweight binary protocol for agents to talk to each other and offload tasks to "Lobster" (OpenClaw) nodes.
- **Agentic Decision Engine:** A pattern-matching brain that handles hardware tasks locally and intelligently forwards complex logic to larger models.
- **Swarm Intelligence:** Deploy 100 Krill agents for the cost (and power) of a single Raspberry Pi.

---

### 🛠 How to Run
1. Install Rust: `rustup target add riscv32imac-unknown-none-elf`
2. Build: `cargo build --release`
3. Flash: `cargo espflash flash --release` (For ESP32 chips)

---

## 🛠 Project Structure

```text
micro-claw/
├── src/
│   ├── main.rs          # The entry point (Zero-latency loop)
│   ├── agent/           # The Decision Engine (The Brain)
│   ├── net/             # Bare-metal networking drivers
│   ├── driver/          # GPIO and Peripheral control
│   └── protocol/        # Krill Mesh Protocol (KMP)
├── Cargo.toml           # Size-optimized configuration
├── memory.x             # Linker script for Flash/RAM layout
└── README.md            