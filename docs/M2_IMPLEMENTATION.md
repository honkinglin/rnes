# M2 Implementation: Sprite Layer & Input

## Overview

M2 milestone implements sprite rendering, OAM DMA, input handling, and sprite zero hit detection. This provides the foundation for interactive graphics and user input in the NES emulator.

## Components Implemented

### 1. Sprite Rendering System

**Location**: `crates/ppu/src/lib.rs`

**Features**:
- Sprite data structure and attributes
- OAM (Object Attribute Memory) management
- Sprite evaluation and rendering
- Sprite-to-background priority
- Sprite flipping (horizontal/vertical)
- Sprite palette support

**Key Components**:
- `Sprite` - Individual sprite data structure
- `SpriteRenderingState` - Sprite rendering state tracking
- `evaluate_sprites()` - Evaluate sprites for current scanline
- `render_sprite()` - Render individual sprite

**Sprite Attributes**:
- Y position (0-255)
- Tile ID (0-255)
- Attributes (palette, priority, flip flags)
- X position (0-255)

### 2. OAM DMA (Object Attribute Memory Direct Memory Access)

**Location**: `crates/ppu/src/lib.rs` and `crates/core/src/bus.rs`

**Features**:
- 256-byte OAM memory
- DMA transfer from system RAM to OAM
- 513-cycle DMA timing (1 dummy read + 256 writes)
- Automatic sprite data loading

**Key Methods**:
- `start_oam_dma()` - Initiate DMA transfer
- `step_oam_dma()` - Step DMA transfer
- `oam_dma_active()` - Check DMA status

**DMA Process**:
1. Write page number to 0x4014
2. PPU reads 256 bytes from RAM page
3. Data is written to OAM (0x00-0xFF)
4. DMA completes after 513 cycles

### 3. Input Handling System

**Location**: `crates/core/src/bus.rs` and `crates/core/src/emulator.rs`

**Features**:
- Controller state management
- Keyboard input mapping
- Button state tracking
- Dual controller support

**Key Components**:
- `ControllerState` - Button state structure
- `Button` - Button enumeration
- `handle_keyboard_input()` - Process keyboard input
- Controller register emulation (0x4016, 0x4017)

**Input Mapping**:
| NES Button | Keyboard | Bit |
|------------|----------|-----|
| A | Z | 0 |
| B | X | 1 |
| Select | Right Shift | 2 |
| Start | Enter | 3 |
| Up | Up Arrow | 4 |
| Down | Down Arrow | 5 |
| Left | Left Arrow | 6 |
| Right | Right Arrow | 7 |

### 4. Sprite Zero Hit Detection

**Location**: `crates/ppu/src/lib.rs`

**Features**:
- Sprite zero hit flag tracking
- PPUSTATUS bit 6 management
- Sprite zero hit timing
- Background collision detection

**Implementation**:
- Tracks when sprite 0 (first sprite) is rendered
- Sets sprite zero hit flag in PPUSTATUS
- Used for sprite-to-background collision detection

## Architecture

### PPU Integration

The sprite system is integrated into the PPU rendering pipeline:

```rust
pub struct PpuState {
    // ... existing fields ...
    pub sprite_rendering: SpriteRenderingState,
    pub oam_dma_active: bool,
    pub oam_dma_cycles: Byte,
    pub oam_dma_addr: Word,
}
```

### Memory Mapping

- **OAM**: 256 bytes (0x00-0xFF) in PPU
- **Sprite Pattern Tables**: 0x0000-0x1FFF (accessed via mapper)
- **Sprite Palettes**: 0x3F10-0x3F1F (4 palettes, 4 colors each)
- **DMA Source**: System RAM pages (0x00-0xFF)

### Rendering Pipeline

1. **Sprite Evaluation** (scanline 0-239, dots 1-64)
   - Scan OAM for sprites visible on current scanline
   - Limit to 8 sprites per scanline
   - Track sprite zero hit

2. **Sprite Rendering** (scanline 0-239, dots 257-320)
   - Fetch sprite pattern data
   - Apply sprite attributes (flip, palette)
   - Render sprite pixels with priority

3. **OAM DMA** (when active)
   - Transfer data from RAM to OAM
   - Block PPU rendering during transfer

## Testing

### Unit Tests

**Location**: `crates/ppu/src/lib.rs`

Tests cover:
- Sprite data structure creation
- Sprite attribute parsing
- OAM DMA functionality
- Input handling

### Integration Demo

**Location**: `examples/m2_demo.rs`

Demonstrates:
- Sprite rendering with test patterns
- Input handling and button mapping
- OAM DMA transfer
- Sprite zero hit detection

## Performance

The implementation provides:
- Efficient sprite evaluation (O(n) where n ≤ 64)
- Optimized sprite rendering with early exit
- Minimal memory allocation during rendering
- Fast input processing

## Limitations

### M2 Scope Limitations

The following features are **not** implemented in M2:
- 16x16 sprite support (only 8x8)
- Advanced sprite timing
- Sprite overflow detection
- Complex sprite-to-sprite priority

### Known Issues

1. **Simplified Timing**: Sprite evaluation timing is simplified
2. **Limited Sprite Size**: Only 8x8 sprites are supported
3. **Basic Priority**: Simple sprite-to-background priority only

## Future Work

### M3 Preparation

M2 provides the foundation for M3 features:
- APU audio system
- Enhanced sprite capabilities
- Advanced input handling

### Optimization Opportunities

1. **Sprite Caching**: Cache sprite pattern data
2. **Batch Rendering**: Render multiple sprites simultaneously
3. **Input Polling**: Optimize input state updates

## Usage Examples

### Basic Sprite Setup

```rust
// Create sprite data
let sprite_data = vec![
    100,  // Y position
    0,    // Tile ID
    0x00, // Attributes (palette 0, no flip, front)
    100,  // X position
];

// Write to RAM
for (i, &byte) in sprite_data.iter().enumerate() {
    bus.write_byte(0x0200 + i as u16, byte)?;
}

// Trigger OAM DMA
bus.write_byte(0x4014, 0x02)?;
```

### Input Handling

```rust
// Handle keyboard input
emulator.handle_keyboard_input(Button::A, true);
emulator.handle_keyboard_input(Button::Up, true);

// Check controller state
let state = emulator.get_controller1_state();
if state.a && state.up {
    println!("A + Up pressed");
}
```

### Sprite Rendering

```rust
// Enable sprite rendering
ppu.write_register(0x2001, 0x10)?; // Enable sprites

// Run emulator to render sprites
emulator.run_cycles(29_780)?; // One frame

// Check frame buffer for sprite pixels
if let Some(frame_buffer) = emulator.get_ppu_frame_buffer() {
    // Process rendered sprites
}
```
