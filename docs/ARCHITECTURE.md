<!--
SPDX-FileCopyrightText: 2025 hexaTune LLC
SPDX-License-Identifier: MIT
-->

# 🏗️ hexaGenMini Architecture

---

## 📬 Questions?

Contact the team at **[info@hexatune.com](mailto:info@hexatune.com)** or open an issue.

---

Built by [hexaTune LLC](https://hexatune.com) · GitHub: [hTuneSys/hexaGenMini](https://github.com/hTuneSys/hexaGenMini) · License: [MIT](https://opensource.org/license/mit/)

---

## Overview

hexaGenMini is a compact signal generator device based on the Raspberry Pi Pico (RP2040) microcontroller. It features a Direct Digital Synthesis (DDS) module for generating precise waveforms, an RGB LED for status indication, and USB MIDI communication for control. The device is designed for integration into music synthesis setups, providing frequency generation capabilities via AT command protocol over MIDI SysEx messages.

## System Architecture

The firmware is written in Rust using the Embassy framework for embedded async programming. It consists of several modules running as concurrent tasks on the RP2040's dual cores:

- **USB Module**: Handles USB MIDI communication
- **AT Command Module**: Parses and dispatches AT commands
- **DDS Module**: Controls the AD985x DDS chip for frequency generation
- **RGB Module**: Manages the WS2812 RGB LED
- **Channel Manager**: Facilitates inter-task communication via async channels

### Core 0 Tasks
- USB Device Task
- USB IO Task
- AT Task
- RGB Task
- Main Loop Task

### Core 1 Tasks
- DDS Task

## System Flow

1. **Initialization**: The main function initializes peripherals, sets up channels, and spawns tasks on both cores.

2. **USB Communication**: USB IO task listens for incoming MIDI packets containing SysEx messages.

3. **AT Command Parsing**: Received SysEx payloads are parsed into AT commands.

4. **Command Dispatch**: The AT dispatcher routes commands to appropriate handlers based on the command name.

5. **Handler Execution**: Handlers perform actions like setting frequency, changing LED color, or triggering firmware updates.

6. **Response Generation**: Results are compiled back into AT response format and sent via USB MIDI.

## AT Command Structure

AT commands follow a specific format for communication with the device:

### Command Format
```
AT+<COMMAND>=<ID>#<PARAM1>#<PARAM2>#...
```

### Query Format
```
AT+<COMMAND>?
```

### Response Format
```
AT+<RESPONSE>=<ID>#<PARAM1>#<PARAM2>#...
```

### Supported Commands

#### VERSION
- **Query**: `AT+VERSION?`
- **Response**: `AT+VERSION=0#v1.0.0`
- **Description**: Returns the firmware version

#### SETRGB
- **Command**: `AT+SETRGB=<ID>#<R>#<G>#<B>`
- **Response**: `AT+DONE=<ID>`
- **Description**: Sets the RGB LED color (R, G, B values 0-255)
- **Example**: `AT+SETRGB=123#255#0#128`

#### RESET
- **Command**: `AT+RESET=<ID>`
- **Description**: Performs a system reset
- **Note**: No response as device resets

#### FWUPDATE
- **Command**: `AT+FWUPDATE=<ID>`
- **Description**: Enters BOOTSEL mode for firmware update
- **Note**: No response as device enters bootloader

#### FREQ
- **Command**: `AT+FREQ=<ID>#<FREQUENCY>#<TIME_MS>`
- **Response**: `AT+DONE=<ID>` or `AT+ERROR=<ID>#<ERROR_CODE>`
- **Description**: Sets DDS frequency with dwell time
- **Parameters**:
  - FREQUENCY: Frequency in Hz (u32)
  - TIME_MS: Dwell time in milliseconds (u32)
- **Example**: `AT+FREQ=456#1000000#5000`

### Error Codes
- E001001: Invalid command
- E001002: DDS busy
- E001003: Invalid UTF-8
- E001004: Invalid SysEx
- E001005: Invalid data length
- E001006: Parameter count error
- E001007: Parameter value error
- E001008: Not a query
- E001009: Unknown command

## Communication Protocol

Commands are sent as MIDI SysEx messages over USB MIDI:

- **SysEx Start**: 0xF0
- **Payload**: UTF-8 encoded AT command string
- **SysEx End**: 0xF7

The USB MIDI implementation uses standard MIDI packet formats for SysEx transmission.

## Hardware Interfaces

- **USB**: Full-speed USB 2.0 for MIDI communication
- **DDS**: AD985x controlled via GPIO bit-banging
- **RGB LED**: WS2812 controlled via PIO
- **Status LED**: Onboard LED for system status

## Task Communication

Inter-task communication uses Embassy async channels with a capacity of 16 messages per channel. The ChannelManager provides typed access to senders and receivers for each module.

## Configuration

System configuration is managed through constants in `hexa_config` module, including version information and DDS availability status.
