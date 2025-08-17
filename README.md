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
- [x] PPU timing model
- [x] Background rendering
- [x] NROM Mapper
- [x] Palette system

### M2: Sprite Layer & Input ✅
- [x] Sprite rendering
- [x] OAM DMA
- [x] Input handling
- [x] Sprite 0 hit

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

## Documentation

- [CPU Implementation](docs/CPU_IMPLEMENTATION.md) - Detailed CPU implementation guide
- [M1 Implementation](docs/M1_IMPLEMENTATION.md) - M1 milestone implementation details
- [PPU Testing](docs/PPU_TESTING.md) - PPU testing framework and procedures
- [Testing Guide](docs/TESTING_GUIDE.md) - Comprehensive testing procedures
- [Progress](docs/PROGRESS.md) - Development progress tracking

## Development

### Requirements

- Rust 1.89+
- Vulkan-capable GPU (for wgpu)

### Testing

```bash
# Run all tests
cargo test

# Download all test ROMs (CPU, PPU, APU)
./scripts/download_all_test_roms.sh

# Or download specific test ROMs:
# Run CPU integration tests (requires test ROMs)
./scripts/download_cpu_test_roms.sh
cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored

# Run PPU integration tests (requires test ROMs)
./scripts/download_ppu_test_roms.sh
cargo test -p rnes-test-suite --test ppu_integration_tests -- --ignored

# Run M2 integration tests (requires test ROMs)
./scripts/download_m2_test_roms.sh
cargo test -p rnes-test-suite --test m2_integration_tests -- --ignored

# Run benchmarks
cargo bench

# Code checks
cargo clippy
cargo fmt
```

#### Test ROMs

This project uses test ROMs for comprehensive CPU and PPU testing. These ROMs are not included in the repository due to copyright considerations. To run integration tests:

**Option 1: Download all test ROMs at once**
```bash
./scripts/download_all_test_roms.sh
```

**Option 2: Download specific test ROMs**
1. Download CPU test ROMs: `./scripts/download_cpu_test_roms.sh`
2. Download PPU test ROMs: `./scripts/download_ppu_test_roms.sh`

**Run tests**
3. Run CPU tests: `cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored`
4. Run PPU tests: `cargo test -p rnes-test-suite --test ppu_integration_tests -- --ignored`

The test ROMs are automatically ignored by Git and will not be committed to the repository.

## License

MIT License
