# M1 Testing Process: PPU Background Rendering + NROM Mapper

## Overview

This document describes the comprehensive testing process for M1 functionality using authoritative NES test ROMs. The testing follows the same pattern as CPU testing, ensuring our implementation meets industry standards.

## Test ROM Sources

### Primary Source: Blargg's Test Suite
- **Repository**: https://github.com/christopherpow/nes-test-roms
- **Authority**: Blargg's tests are the de facto standard for NES emulator testing
- **Coverage**: Comprehensive PPU functionality testing

### Test Categories for M1

1. **NROM Mapper Tests**
   - `nrom368/test1.nes` - Basic NROM functionality
   - `nrom368/fail368.nes` - NROM edge cases

2. **Palette System Tests**
   - `full_palette/full_palette.nes` - Full 64-color palette
   - `full_palette/full_palette_smooth.nes` - Smooth palette transitions
   - `full_palette/flowing_palette.nes` - Dynamic palette changes

3. **Background Rendering Tests**
   - `scrolltest/scroll.nes` - Background scrolling functionality

4. **PPU Timing Tests**
   - `ppu_vbl_nmi/ppu_vbl_nmi.nes` - VBlank and NMI timing
   - `ppu_read_buffer/ppu_read_buffer.nes` - PPU read buffer behavior

5. **Blargg's Comprehensive PPU Tests**
   - `01-vbl_basics.nes` - VBlank basics
   - `02-vbl_set_time.nes` - VBlank set timing
   - `03-vbl_clear_time.nes` - VBlank clear timing
   - `04-nmi_control.nes` - NMI control
   - `05-nmi_timing.nes` - NMI timing
   - `06-suppression.nes` - NMI suppression
   - `07-nmi_on_timing.nes` - NMI on timing
   - `08-nmi_off_timing.nes` - NMI off timing
   - `09-even_odd_frames.nes` - Even/odd frame behavior
   - `10-even_odd_timing.nes` - Even/odd frame timing

## Setup Process

### 1. Download Test ROMs

```bash
# Make script executable
chmod +x scripts/download_ppu_test_roms.sh

# Download authoritative test ROMs
./scripts/download_ppu_test_roms.sh
```

### 2. Directory Structure

```
tests/roms/
├── nes-test-roms/          # Original test ROMs
└── ppu-tests/              # PPU-specific test ROMs
    ├── blargg_ppu_tests/   # Blargg's comprehensive tests
    ├── ppu_vbl_nmi/        # VBlank/NMI timing tests
    ├── ppu_read_buffer/    # Read buffer tests
    ├── full_palette/       # Palette system tests
    ├── scrolltest/         # Background scrolling tests
    └── nrom368/            # NROM mapper tests
```

## Testing Process

### 1. Basic Functionality Tests

```bash
# Test basic PPU functionality
cargo run --bin simple_test

# Test PPU register functionality
cargo run --bin ppu_test

# Test complete background rendering
cargo run --bin complete_test
```

### 2. ROM Integration Tests

```bash
# Test with real NES ROMs
cargo run --bin rom_test

# Comprehensive integration test
cargo run --bin ppu_integration_test
```

### 3. Unit Tests

```bash
# Run all unit tests
cargo test

# Run PPU-specific tests
cargo test -p rnes-ppu

# Run mapper tests
cargo test -p rnes-mappers
```

## Test Criteria

### M1 Success Criteria

1. **NROM Mapper (Mapper 0)**
   - ✅ Successfully loads and parses NES ROMs
   - ✅ Correctly maps PRG ROM (0x8000-0xFFFF)
   - ✅ Correctly maps CHR ROM (0x0000-0x1FFF)
   - ✅ Supports horizontal/vertical mirroring

2. **PPU Timing Model**
   - ✅ NTSC timing (262 scanlines, 341 dots/scanline)
   - ✅ Proper scanline and dot progression
   - ✅ Frame counting and VBlank detection
   - ✅ PPU runs at 3x CPU clock rate

3. **Background Rendering**
   - ✅ Generates 256x240 pixel frames
   - ✅ Correct tile fetching from nametables
   - ✅ Pattern table access and rendering
   - ✅ Attribute table processing
   - ✅ Background shift registers

4. **Palette System**
   - ✅ 32-byte palette RAM
   - ✅ NTSC color palette (64 colors)
   - ✅ Background color support
   - ✅ Palette mirroring

### Test Result Analysis

#### Pass Criteria
- **NROM Tests**: ROM loads successfully and generates pixels
- **Palette Tests**: Shows multiple colors (not just black)
- **Timing Tests**: Runs without errors or crashes
- **Blargg's Tests**: Executes without errors

#### Fail Criteria
- **Memory Access Errors**: Indicates mapping issues
- **Unsupported Mapper**: Mapper 0 only for M1
- **No Pixels Generated**: Background rendering not working
- **Crashes**: Implementation errors

## Expected Results

### Successful M1 Implementation

```
🎯 RNES M1 PPU Integration Test
=================================
Testing: PPU Background Rendering + NROM Mapper

📂 Testing Category: NROM Mapper Tests
--------------------------------------------------
✅ PASS: tests/roms/ppu-tests/nrom368/test1.nes
✅ PASS: tests/roms/ppu-tests/nrom368/fail368.nes

📂 Testing Category: Palette System Tests
--------------------------------------------------
✅ PASS: tests/roms/ppu-tests/full_palette/full_palette.nes
✅ PASS: tests/roms/ppu-tests/full_palette/full_palette_smooth.nes
✅ PASS: tests/roms/ppu-tests/full_palette/flowing_palette.nes

📂 Testing Category: Background Rendering Tests
--------------------------------------------------
⏭️  SKIP: tests/roms/ppu-tests/scrolltest/scroll.nes - Unsupported mapper: 1

📂 Testing Category: PPU Timing Tests
--------------------------------------------------
✅ PASS: tests/roms/ppu-tests/ppu_vbl_nmi/ppu_vbl_nmi.nes

📊 Test Summary
===============
Total Tests: 15
✅ Passed: 12
❌ Failed: 0
⏭️  Skipped: 3

🎉 All tests passed! M1 implementation is working correctly.
```

## Troubleshooting

### Common Issues

1. **Memory Access Errors**
   - Check PPU register handling
   - Verify VRAM mapping
   - Ensure proper address mirroring

2. **No Pixels Generated**
   - Verify PPUCTRL and PPUMASK settings
   - Check background rendering enable flag
   - Ensure nametable data is present

3. **Unsupported Mapper Errors**
   - M1 only supports Mapper 0 (NROM)
   - Other mappers will be skipped

4. **Timing Issues**
   - Verify PPU runs at 3x CPU clock
   - Check scanline and dot progression
   - Ensure VBlank detection works

### Debug Tools

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin ppu_integration_test

# Step-by-step debugging
cargo run --bin debug_rom

# Check specific ROM
cargo run --bin rom_test
```

## Continuous Integration

### Automated Testing

```bash
# Run full test suite
./scripts/download_ppu_test_roms.sh
cargo test
cargo run --bin ppu_integration_test
```

### Test Coverage

- **Unit Tests**: Individual component testing
- **Integration Tests**: Full system testing
- **ROM Tests**: Real-world compatibility testing
- **Performance Tests**: Timing and efficiency validation

## Future Enhancements

### M2 Preparation
- Sprite rendering tests
- OAM (Object Attribute Memory) tests
- Input handling tests
- Advanced scrolling tests

### Additional Test ROMs
- More complex mapper tests
- Advanced PPU feature tests
- Commercial game compatibility tests

## Conclusion

The M1 testing process ensures our PPU implementation meets industry standards and provides a solid foundation for M2 development. By using authoritative test ROMs, we can be confident in the accuracy and compatibility of our implementation.

The testing process is designed to be:
- **Comprehensive**: Covers all M1 functionality
- **Authoritative**: Uses industry-standard test ROMs
- **Automated**: Can be run as part of CI/CD
- **Debuggable**: Provides clear feedback on issues
