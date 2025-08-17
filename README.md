# RNES - NES Emulator

A NES (Nintendo Entertainment System) emulator written in Rust.

## Project Overview

RNES is a high-performance, cross-platform NES emulator with a modular design, supporting multiple Mapper types, and providing both native and Web frontend.

## Tech Stack

- **Language**: Rust 1.78+ (Edition 2021)
- **Architecture**: Workspace multi-crate design
- **Graphics**: wgpu (cross-platform GPU rendering)
- **Audio**: cpal (cross-platform audio)
- **Input**: winit + gilrs (keyboard/gamepad support)
- **Serialization**: serde + bincode

## Project Structure

```
crates/
  cpu6502/        # 6502 CPU core
  ppu/            # Picture Processing Unit
  apu/            # Audio Processing Unit
  mappers/        # Mapper implementations
  cartridge/      # Cartridge management
  core/           # Bus, DMA, scheduler
  common/         # Shared types and utilities
frontend/
  native/         # Native application
  web/            # Web frontend (planned)
```

## Development Roadmap

### M0: CPU Ready ✅
- [ ] 6502 instruction set implementation
- [ ] Addressing modes
- [ ] Cycle counting
- [ ] NMI/IRQ handling

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

- Rust 1.78+
- Vulkan-capable GPU (for wgpu)

### Testing

```bash
# Run all tests
cargo test

# Run benchmarks
cargo bench

# Code checks
cargo clippy
cargo fmt
```

## License

MIT License
