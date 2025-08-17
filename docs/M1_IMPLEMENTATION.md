# M1 Implementation: PPU Background Rendering + NROM Mapper

## Overview

M1 milestone implements the core PPU (Picture Processing Unit) functionality with background rendering and NROM mapper support. This provides the foundation for displaying graphics in the NES emulator.

## Components Implemented

### 1. NROM Mapper (Mapper 0)

**Location**: `crates/mappers/src/lib.rs`

**Features**:
- Memory mapping for PRG ROM (0x8000-0xFFFF)
- Memory mapping for CHR ROM/RAM (0x0000-0x1FFF)
- PRG RAM support (0x6000-0x7FFF)
- Proper mirroring for 16KB and 32KB PRG ROM configurations

**Key Methods**:
- `read_prg()` - Read from PRG ROM/RAM
- `write_prg()` - Write to PRG RAM
- `read_chr()` - Read from CHR ROM/RAM
- `write_chr()` - Write to CHR RAM
- `mirroring()` - Get mirroring configuration

### 2. PPU Timing Model

**Location**: `crates/ppu/src/lib.rs`

**Features**:
- NTSC timing (262 scanlines, 341 dots per scanline)
- Proper scanline and dot progression
- Frame counting
- VBlank detection and signaling

**Key Components**:
- `PpuState` - Internal PPU state tracking
- `step()` - Single dot advancement
- Scanline phase handling (visible, pre-render, vblank)

### 3. Background Rendering

**Location**: `crates/ppu/src/lib.rs`

**Features**:
- Nametable tile fetching
- Pattern table access
- Attribute table processing
- Background shift registers
- Fine X scrolling support
- Pixel color generation

**Key Methods**:
- `render_background_scanline()` - Render one scanline
- `fetch_background_tiles()` - Fetch tile data
- `get_background_pixel()` - Generate pixel colors
- `load_background_registers()` - Update shift registers

### 4. Palette System

**Location**: `crates/ppu/src/lib.rs` and `crates/common/src/constants.rs`

**Features**:
- 32-byte palette RAM
- NTSC color palette (64 colors)
- Background color support
- Palette mirroring

**Key Components**:
- `NES_PALETTE` - Standard NES color palette
- Palette RAM management
- Color index to RGB conversion

## Architecture

### PPU Integration

The PPU is integrated into the emulator through the `Bus` component:

```rust
pub struct Bus {
    pub cartridge: Option<Cartridge>,
    pub ppu: Option<Ppu>,
    pub ram: [Byte; RAM_SIZE],
    // ... other components
}
```

### Memory Mapping

- **PPU Registers**: 0x2000-0x2007
- **Pattern Tables**: 0x0000-0x1FFF (accessed via mapper)
- **Nametables**: 0x2000-0x3EFF (with mirroring)
- **Palette RAM**: 0x3F00-0x3F1F (with mirroring)

### Timing Synchronization

The PPU runs at 3x the CPU clock rate:
- CPU: 1.789773 MHz
- PPU: 5.369319 MHz (3x CPU)
- Each CPU cycle triggers 3 PPU cycles

## Testing

### Unit Tests

**Location**: `crates/ppu/src/lib.rs`

Tests cover:
- PPU creation and initialization
- Timing progression
- Basic functionality verification

### Integration Demo

**Location**: `examples/m1_demo.rs`

Demonstrates:
- ROM loading with NROM mapper
- PPU initialization
- Background rendering
- Frame generation
- VBlank detection

## Performance

The implementation generates full frames (256x240 pixels) with:
- Proper timing synchronization
- Efficient memory access patterns
- Minimal allocation overhead

## Limitations

### M1 Scope Limitations

The following features are **not** implemented in M1:
- Sprite rendering (OAM)
- Sprite zero hit detection
- Sprite overflow detection
- Advanced scrolling effects
- Sprite-to-background priority

### Known Issues

1. **Simplified Tile Fetching**: The tile fetching logic is simplified and may not match exact NES timing
2. **Limited Pattern Support**: Only basic pattern table access is implemented
3. **No Sprite Support**: Sprites are completely unimplemented

## Future Work

### M2 Preparation

M1 provides the foundation for M2 features:
- Sprite rendering system
- OAM (Object Attribute Memory) support
- Input handling integration
- Enhanced scrolling effects

### Optimization Opportunities

1. **Memory Access**: Optimize VRAM access patterns
2. **Rendering Pipeline**: Improve pixel generation efficiency
3. **Timing Accuracy**: Enhance cycle-accurate timing

## Usage Example

```rust
use rnes_core::Emulator;
use rnes_cartridge::Cartridge;

// Load ROM
let cartridge = Cartridge::from_file("game.nes")?;
let mut emulator = Emulator::new();
emulator.load_rom(cartridge)?;

// Run emulator
emulator.start();
emulator.run_cycles(29_780); // One frame

// Access frame buffer
if let Some(frame_buffer) = emulator.get_ppu_frame_buffer() {
    // Process 256x240 pixel frame
    for pixel in frame_buffer {
        // Handle pixel data
    }
}
```

## Conclusion

M1 successfully implements the core PPU functionality with background rendering and NROM mapper support. The implementation provides a solid foundation for the NES emulator and demonstrates proper timing, memory management, and graphics generation capabilities.

The milestone is complete and ready for M2 development.
