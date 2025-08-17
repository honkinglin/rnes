# M4 Implementation: Common Mappers

## Overview

M4 milestone implements support for common NES mappers beyond the basic NROM mapper. This includes MMC1, UxROM, CNROM, and AOROM mappers, which cover a significant portion of NES games and provide the foundation for more complex mapper implementations.

## Components Implemented

### 1. MMC1 Mapper (Mapper 1)

**Location**: `crates/mappers/src/lib.rs`

**Features**:
- 16KB or 32KB PRG ROM banks with configurable banking modes
- 4KB CHR ROM/RAM banks with 4KB or 8KB modes
- Battery backup support for save games
- Configurable mirroring (Single Screen A/B, Horizontal, Vertical)
- Serial shift register for register writes
- PRG RAM support

**Key Implementation Details**:
- **Serial Shift Register**: MMC1 uses a 5-bit shift register for writing to internal registers
- **Bank Modes**: Supports 4 different PRG banking modes (32KB, Fixed First, Fixed Last, Switchable)
- **CHR Modes**: Supports both 4KB and 8KB CHR banking modes
- **Register Writes**: Writes to PRG ROM space are interpreted as register writes

**Key Methods**:
- `write_register()` - Handle serial shift register writes
- `get_prg_bank()` - Calculate PRG bank based on current mode
- `get_chr_bank()` - Calculate CHR bank based on current mode

### 2. UxROM Mapper (Mapper 2)

**Location**: `crates/mappers/src/lib.rs`

**Features**:
- 16KB PRG ROM banks (switchable)
- Fixed last 16KB of PRG ROM
- 8KB CHR ROM/RAM
- Simple bank switching via writes to PRG ROM space

**Key Implementation Details**:
- **Bank Selection**: Writes to any PRG ROM address select the bank for 0x8000-0xBFFF
- **Fixed Bank**: Last 16KB (0xC000-0xFFFF) always maps to the last bank
- **Simple Design**: No complex banking modes or internal state

### 3. CNROM Mapper (Mapper 3)

**Location**: `crates/mappers/src/lib.rs`

**Features**:
- 32KB PRG ROM (fixed)
- 8KB CHR ROM banks (switchable)
- Simple CHR bank switching via writes to PRG ROM space

**Key Implementation Details**:
- **CHR Bank Selection**: Writes to any PRG ROM address select the CHR bank
- **Fixed PRG**: PRG ROM is always mapped as 32KB without banking
- **CHR ROM**: Typically uses CHR ROM, so writes to CHR space are ignored

### 4. AOROM Mapper (Mapper 7)

**Location**: `crates/mappers/src/lib.rs`

**Features**:
- 32KB PRG ROM banks (switchable)
- 8KB CHR ROM/RAM
- Simple bank switching
- Configurable mirroring (Single Screen A/B)

**Key Implementation Details**:
- **Bank Selection**: Writes to any PRG ROM address select the bank
- **Mirroring Control**: Bit 4 of the bank select value controls mirroring
- **32KB Banks**: Each bank is 32KB, covering the entire PRG ROM space

## Architecture

### Mapper Factory Pattern

The `create_mapper()` function in `crates/mappers/src/lib.rs` implements a factory pattern:

```rust
pub fn create_mapper(cartridge: Cartridge) -> RnesResult<Box<dyn Mapper>> {
    match cartridge.mapper_number() {
        0 => Ok(Box::new(NromMapper::new(cartridge))),
        1 => Ok(Box::new(Mmc1Mapper::new(cartridge))),
        2 => Ok(Box::new(UxromMapper::new(cartridge))),
        3 => Ok(Box::new(CnromMapper::new(cartridge))),
        7 => Ok(Box::new(AoromMapper::new(cartridge))),
        mapper => Err(rnes_common::RnesError::UnsupportedMapper(mapper))
    }
}
```

### Memory Mapping

Each mapper implements the `Mapper` trait with consistent memory mapping:

- **PRG ROM**: 0x8000-0xFFFF (with bank switching)
- **PRG RAM**: 0x6000-0x7FFF (if present)
- **CHR ROM/RAM**: 0x0000-0x1FFF (with bank switching)

### Bank Switching Mechanisms

1. **MMC1**: Serial shift register with 5-bit writes
2. **UxROM**: Direct bank selection via writes to PRG space
3. **CNROM**: Direct CHR bank selection via writes to PRG space
4. **AOROM**: Direct bank selection with mirroring control

## Testing

### Unit Tests

**Location**: `crates/test-suite/src/m4_integration_tests.rs`

Tests cover:
- Mapper creation and initialization
- Basic functionality verification
- Bank switching behavior
- Memory access patterns

### Integration Tests

**Location**: `crates/test-suite/src/m4_integration_tests.rs`

Integration tests (marked with `#[ignore]`) for:
- MMC1 mapper functionality
- UxROM mapper functionality
- CNROM mapper functionality
- AOROM mapper functionality

### Demo Application

**Location**: `examples/m4_demo.rs`

Demonstrates:
- Loading ROMs with different mappers
- Basic emulator operation with each mapper
- Frame generation and rendering
- Mapper-specific behavior verification

## Performance

The mapper implementations are designed for:
- **Efficient Memory Access**: Direct bank calculation without complex lookups
- **Minimal Overhead**: Simple arithmetic for bank switching
- **Memory Safety**: Proper bounds checking and error handling

## Limitations

### M4 Scope Limitations

The following features are **not** implemented in M4:
- Save state persistence for battery-backed games
- Advanced mapper features (IRQ generation, scanline counting)
- MMC3 and other complex mappers
- Four-screen mirroring support

### Known Issues

1. **Save System**: Battery backup save/load functionality is not implemented
2. **Advanced Features**: Some mapper-specific features may not be fully accurate
3. **Timing**: Mapper register write timing may not match exact NES behavior

## Future Work

### M5 Preparation

M4 provides the foundation for M5 features:
- MMC3 mapper implementation
- IRQ generation and handling
- Scanline counting
- More complex banking modes

### Save System Implementation

Future work should include:
- Battery backup save/load functionality
- Save state serialization
- Persistent storage integration

### Optimization Opportunities

1. **Bank Calculation**: Optimize bank switching arithmetic
2. **Memory Access**: Improve memory access patterns
3. **Register Writes**: Optimize register write handling

## Usage Examples

### Creating a Mapper

```rust
use rnes_cartridge::Cartridge;
use rnes_mappers::create_mapper;

let cartridge = Cartridge::from_file("game.nes")?;
let mapper = create_mapper(cartridge)?;
```

### Running M4 Demo

```bash
cargo run --example m4_demo
```

### Running M4 Tests

```bash
# Setup test ROMs
./scripts/download_m4_test_roms.sh

# Run integration tests
cargo test -p rnes-test-suite --test m4_integration_tests -- --ignored
```

## References

- [NES Mapper Documentation](https://wiki.nesdev.com/w/index.php/Mapper)
- [MMC1 Technical Reference](https://wiki.nesdev.com/w/index.php/MMC1)
- [UxROM Technical Reference](https://wiki.nesdev.com/w/index.php/UxROM)
- [CNROM Technical Reference](https://wiki.nesdev.com/w/index.php/CNROM)
- [AOROM Technical Reference](https://wiki.nesdev.com/w/index.php/AOROM)
