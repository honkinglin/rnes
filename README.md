# RNES - NES Emulator

A NES (Nintendo Entertainment System) emulator written in Rust.

## Project Overview

RNES is a high-performance, cross-platform NES emulator with a modular design, supporting multiple Mapper types, and providing both native and Web frontend.

## Tech Stack

- **Language**: Rust 1.89+
- **Architecture**: Workspace multi-crate design
- **Graphics**: wgpu (cross-platform GPU rendering)
- **Audio**: cpal (cross-platform audio)
- **Input**: winit + gilrs (keyboard/gamepad support)
- **Serialization**: serde + bincode

## Project Structure

```
crates/
  common/         # Shared types, traits, and utilities
  cpu6502/        # 6502 CPU implementation (151 instructions)
  ppu/            # Picture Processing Unit (planned)
  apu/            # Audio Processing Unit (planned)
  mappers/        # Memory mapper implementations (planned)
  cartridge/      # Cartridge and ROM management
  core/           # Bus, DMA, and emulator core (planned)
  test-suite/     # Integration testing framework
frontend/
  native/         # Native desktop application
  web/            # Web frontend (planned)
```

## Development Roadmap

### M0: CPU Ready ✅
- [x] 6502 instruction set implementation
- [x] Addressing modes
- [x] Cycle counting
- [x] NMI/IRQ handling

### M1: PPU Background Rendering + NROM
- [ ] PPU timing model
- [ ] Background rendering
- [ ] NROM Mapper
- [ ] Palette system

### M2: Sprite Layer & Input
- [ ] Sprite rendering
- [ ] OAM DMA
- [ ] Input handling
- [ ] Sprite 0 hit

### M3: APU Audio
- [ ] Audio channel implementation
- [ ] Mixing system
- [ ] Audio output

### M4: Common Mappers
- [ ] MMC1, UxROM, CNROM, AOROM
- [ ] Save system

### M5: MMC3 + IRQ
- [ ] MMC3 implementation
- [ ] Scanline IRQ

### M6: Tools & Experience
- [ ] Save states
- [ ] Debugger
- [ ] Configuration system

### M7: Accuracy & Polish
- [ ] PPU timing optimization
- [ ] Web frontend
- [ ] Performance optimization

## Building and Running

```bash
# Clone the project
git clone https://github.com/yourusername/rnes.git
cd rnes

# Build
cargo build --release

# Run (requires ROM file)
cargo run --release -- path/to/rom.nes
```

## Controls

| NES Button | Keyboard |
|------------|----------|
| D-pad | Arrow Keys |
| Start | Enter |
| Select | Right Shift |
| A | Z |
| B | X |
| A (Turbo) | A |
| B (Turbo) | S |
| Reset | R |

## Supported Mappers

- [ ] NROM (0)
- [ ] MMC1 (1)
- [ ] UxROM (2)
- [ ] CNROM (3)
- [ ] MMC3 (4)
- [ ] AOROM (7)

## Development

### Requirements

- Rust 1.89+
- Vulkan-capable GPU (for wgpu)

### Testing

```bash
# Run all tests
cargo test

# Run CPU integration tests (requires test ROMs)
./scripts/download_test_roms.sh
cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored

# Run benchmarks
cargo bench

# Code checks
cargo clippy
cargo fmt
```

#### Test ROMs

This project uses test ROMs for comprehensive CPU testing. These ROMs are not included in the repository due to copyright considerations. To run integration tests:

1. Download test ROMs: `./scripts/download_test_roms.sh`
2. Run tests: `cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored`

The test ROMs are automatically ignored by Git and will not be committed to the repository.

## License

MIT License
