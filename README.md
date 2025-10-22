<!--
SPDX-FileCopyrightText: 2025 hexaTune LLC
SPDX-License-Identifier: MIT
-->

# 🎛️ hexaGenMini

[![GitHub](https://img.shields.io/badge/GitHub-hTuneSys/hexagenmini-blue)](https://github.com/hTuneSys/hexaGenMini)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.86.0-orange)](https://www.rust-lang.org/)
[![Embassy](https://img.shields.io/badge/Embassy-0.9.0-red)](https://embassy.dev/)

**hexaGenMini** is a compact, USB-powered signal generator designed for music synthesis and electronic experimentation. Built on the Raspberry Pi Pico (RP2040), it features Direct Digital Synthesis (DDS) for precise frequency generation, RGB status indication, and USB MIDI control via AT commands.

Developed by **hexaTune LLC** and the **hexaTeam**, led by **Husamettin ARABACI**.

## ✨ Features

- **Precise Frequency Generation**: AD985x DDS chip supporting frequencies up to 125MHz
- **USB MIDI Control**: Send AT commands via MIDI SysEx for remote control
- **RGB Status LED**: Visual feedback for device state and DDS availability
- **Firmware Updates**: Built-in BOOTSEL mode for easy firmware flashing
- **Cross-Platform**: Works with any USB MIDI-compatible host
- **Mobile App**: Control via dedicated iOS/Android app ([hexaGenApp](https://github.com/hTuneSys/hexaGenApp))
- **Open Source**: Fully open hardware and software under MIT license

## 📱 Mobile App

Control your hexaGenMini wirelessly with our companion mobile app:

- **iOS & Android Support**
- **Real-time Frequency Control**
- **Preset Management**
- **Visual Waveform Display**

[![Get it on Google Play](https://img.shields.io/badge/Get%20it%20on-Google%20Play-green)](https://play.google.com/store/apps/developer?id=hexaTune+LLC)
[![Download on the App Store](https://img.shields.io/badge/Download%20on-the%20App%20Store-black)](https://apps.apple.com/us/developer/hexatune-llc/id1234567890)

**Source Code**: [hTuneSys/hexaGenApp](https://github.com/hTuneSys/hexaGenApp)

## 🚀 Quick Start

1. **Connect** your hexaGenMini via USB
2. **Flash Firmware**:
   ```bash
   cd firmware
   cargo run
   ```
3. **Send AT Commands** via MIDI SysEx:
   ```
   AT+FREQ=1#1000000#1000  # Set 1MHz for 1 second
   AT+SETRGB=2#255#0#128   # Set LED to purple
   ```

## 📦 Installation

### Prerequisites

- **Rust** (1.86.0+): [Install rustup](https://rustup.rs/)
- **picotool**: For flashing RP2040
- **Node.js & pnpm**: For development tools
- **KiCad** (optional): For hardware modifications
- **FreeCAD** (optional): For enclosure modifications

### Setup

```bash
# Clone repository
git clone https://github.com/hTuneSys/hexaGenMini.git
cd hexaGenMini

# Install development dependencies
pnpm install
pnpm prepare  # Setup husky pre-commit hooks

# Install Rust targets
rustup target add thumbv6m-none-eabi
```

### Flashing Firmware

```bash
cd firmware
cargo run
```

Put your device in BOOTSEL mode (hold BOOTSEL while plugging in) when prompted.

## 🎯 Usage

### AT Command Protocol

hexaGenMini uses AT commands sent via MIDI SysEx messages:

#### Supported Commands

- `AT+VERSION?` - Get firmware version
- `AT+SETRGB=<ID>#<R>#<G>#<B>` - Set RGB LED color
- `AT+FREQ=<ID>#<FREQ>#<TIME_MS>` - Generate frequency with dwell time
- `AT+RESET=<ID>` - System reset
- `AT+FWUPDATE=<ID>` - Enter firmware update mode

#### Example Usage

```bash
# Query version
AT+VERSION?

# Set frequency to 440Hz for 5 seconds
AT+FREQ=1#440#5000

# Set LED to red
AT+SETRGB=2#255#0#0
```

### Hardware Connections

- **USB**: Power and MIDI communication
- **SMA Output**: DDS signal output
- **RGB LED**: Status indication
- **BOOTSEL**: Firmware update mode

## 🏗️ Architecture

The firmware is built with Rust and Embassy, running concurrent async tasks:

- **USB Task**: MIDI communication handling
- **AT Dispatcher**: Command parsing and routing
- **DDS Task**: Frequency generation control
- **RGB Task**: LED management
- **Main Loop**: Status monitoring

For detailed architecture information, see [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## 📁 Project Structure

```
hexaGenMini/
├── firmware/          # Rust firmware (Embassy framework)
├── hardware/          # KiCad PCB designs
├── mechanic/          # FreeCAD enclosure designs
├── docs/             # Documentation
├── .github/          # CI/CD workflows
└── README.md
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch from `develop`
3. Make your changes
4. Submit a pull request
5. Follow conventional commit format

### Areas for Contribution

- Firmware enhancements
- Hardware improvements
- Documentation
- Mobile app features
- Testing and CI/CD

## 📄 Documentation

- [Getting Started](docs/GETTING_STARTED.md) - Setup guide
- [Architecture](docs/ARCHITECTURE.md) - System design
- [Project Structure](docs/PROJECT_STRUCTURE.md) - Repository overview
- [Contributing](docs/CONTRIBUTING.md) - How to contribute
- [FAQ](docs/FAQ.md) - Common questions

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

- **Website**: [hexatune.com](https://hexatune.com)
- **Email**: [info@hexatune.com](mailto:info@hexatune.com)
- **GitHub**: [hTuneSys](https://github.com/hTuneSys)
- **Issues**: [GitHub Issues](https://github.com/hTuneSys/hexaGenMini/issues)

## 🙏 Acknowledgments

Built with ❤️ by the hexaTeam at hexaTune LLC.

Special thanks to the Rust embedded community and Embassy framework developers.

---

**hexaGenMini** - Precision meets portability in signal generation.

