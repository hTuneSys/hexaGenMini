# SPDX-FileCopyrightText: 2025 hexaTune LLC
# SPDX-License-Identifier: MIT
# ===== User config =====
BIN        ?= hexagenmini
TRIPLE     ?= thumbv6m-none-eabi
PROFILE    ?= release
TARGET_DIR ?= target
PICOTOOL   ?= picotool
CARGO      ?= cargo
# =======================

export CARGO_TARGET_DIR := $(abspath $(TARGET_DIR))

ifeq ($(PROFILE),release)
  BUILD_FLAGS := --release
else
  BUILD_FLAGS :=
endif

BINPATH := $(TARGET_DIR)/$(TRIPLE)/$(PROFILE)/$(BIN)
ELF     := $(BINPATH).elf

.PHONY: all build elf load check-device clippy clean help

all: load

build: clean
	@echo "==> Building ($(PROFILE)) to $(CARGO_TARGET_DIR) for $(TRIPLE)"
	@$(CARGO) build --target $(TRIPLE) $(BUILD_FLAGS)

elf: build
	@echo "==> Copying ELF to: $(ELF)"
	@cp -f "$(BINPATH)" "$(ELF)"

check-device:
	@echo "==> Checking RP2040 in BOOTSEL (2e8a:0003)"
	@lsusb -d 2e8a:0003 >/dev/null || (echo "!! RP2040 BOOTSEL not found."; exit 1)

load: check-device elf
	@echo "==> Loading via picotool"
	@$(PICOTOOL) load -x -v -f "$(ELF)"
	@echo "==> Done."

clippy:
	@echo "==> Clippy"
	@$(CARGO) clippy --target $(TRIPLE) $(BUILD_FLAGS) -- -D warnings

clean:
	@echo "==> Cleaning $(CARGO_TARGET_DIR)"
	@$(CARGO) clean

monitor:
	@PORT=$$(ls /dev/ttyACM* 2>/dev/null | head -n1); \
	if [ -z "$$PORT" ]; then echo "No /dev/ttyACM* found. Is the device in application mode?"; exit 1; fi; \
	echo "Opening $$PORT at 115200 baud..."; \
	picocom -b 115200 $$PORT

help:
	@echo "Targets:"
	@echo "  make build       - Build firmware ($(PROFILE) mode)"
	@echo "  make elf         - Generate .elf file"
	@echo "  make load        - Load firmware to device (requires BOOTSEL mode)"
	@echo "  make clippy      - Run Clippy"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make monitor     - Open serial monitor (115200 baud)"
