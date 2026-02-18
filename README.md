<p align="center">
  <img src="https://i.postimg.cc/Z0tX6v9N/file-000000003e707209bf553384c86fbba0.png" width="300" alt="MicroClaw Logo">
</p>

# MicroClaw (The Krill) 🦐
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

## 🚀 Getting Started

MicroClaw runs on bare metal, so you need the Rust Embedded Toolchain. These instructions are optimized for **RISC-V** chips (like the **ESP32-C3**, **BL602**, or **ESP32-H2**).

### 1. Prerequisites
Install Rust and the specific hardware target:
```bash
# Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the RISC-V target (standard for Krill agents)
rustup target add riscv32imac-unknown-none-elf

# Install the flasher tool (for ESP32-C3/RISC-V boards)
cargo install cargo-espflash
```

### 2. Clone & Build
```bash
git clone https://github.com/your-username/micro-claw.git
cd micro-claw

# Build for Release (Size Optimization Active)
cargo build --release
```
*You will see the binary generated in `target/riscv32imac-unknown-none-elf/release/`*

### 3. Flash to Hardware
Connect your device via USB and run:
```bash
# Flash and open the serial monitor to see the Krill Brain thinking
cargo espflash flash --release --monitor
```

### 4. Verify
Once flashed, you should see logs indicating the **Krill Agent** is active:
```text
[Krill] Hardware Initialized.
[Krill] Brain Active (ID: 0x01).
[Net] Listening for KMP signals on port 8888...
```

---

## 🛠 Project Structure

```text
micro-claw/
├── .cargo/
│   └── config.toml      # RISC-V build target configuration
├── src/
│   ├── main.rs          # The entry point (Zero-latency loop)
│   ├── agent/           # The Decision Engine (The Brain)
│   ├── net/             # Bare-metal networking drivers
│   ├── driver/          # GPIO and Peripheral control
│   └── protocol/        # Krill Mesh Protocol (KMP)
├── Cargo.toml           # Size-optimized configuration
├── memory.x             # Linker script for Flash/RAM layout
└── README.md            # Documentation
```

---

## 🤝 Contributing
MicroClaw is Open Source. We welcome contributions to the **Krill Mesh Protocol** and hardware drivers for new RISC-V chips.