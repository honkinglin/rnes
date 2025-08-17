# PPU Testing Guide

This document describes the PPU testing framework for the RNES emulator, similar to the CPU testing framework.

## Overview

The PPU testing framework provides comprehensive testing capabilities for the Picture Processing Unit (PPU) component of the NES emulator. It includes:

- **PPU Test Runner**: A framework for running PPU-specific test ROMs
- **Integration Tests**: Automated tests using authoritative test ROMs
- **Frame Analysis**: Tools for analyzing PPU output and rendering correctness

## Architecture

### PPU Test Runner (`PpuTestRunner`)

Located in `crates/test-suite/src/ppu_test_runner.rs`, the PPU test runner provides:

- ROM loading and emulation execution
- Test result analysis (completion, timeout, infinite loop detection)
- Frame buffer analysis capabilities
- Configurable test parameters (max cycles, max frames)

### PPU Integration Tests

Located in `crates/test-suite/src/ppu_integration_tests.rs`, these tests cover:

- **Blargg's PPU Test Suite**: Comprehensive PPU functionality tests
- **VBL NMI Tests**: VBlank and NMI timing verification
- **Read Buffer Tests**: PPU read buffer behavior validation
- **Full Palette Tests**: Complete palette system testing
- **NROM Mapper Tests**: Mapper-specific functionality
- **Background Scrolling Tests**: Scrolling implementation verification

## Test ROMs

The framework uses authoritative test ROMs from various sources:

### Blargg's PPU Tests
- `vbl_clear_time.nes` - VBlank clear timing
- `palette_ram.nes` - Palette RAM functionality
- `sprite_ram.nes` - Sprite RAM operations
- `vram_access.nes` - VRAM access patterns
- `power_up_palette.nes` - Power-up palette state

### Additional Test Categories
- **PPU VBL NMI**: VBlank and NMI timing tests
- **PPU Read Buffer**: Read buffer behavior tests
- **Full Palette**: Complete palette rendering tests
- **Scroll Tests**: Background scrolling functionality
- **NROM 368**: NROM mapper specific tests

## Usage

### Running PPU Tests

1. **Download Test ROMs**:
   ```bash
   ./scripts/download_ppu_test_roms.sh
   ```

2. **Run Basic PPU Tests**:
   ```bash
   cargo test -p rnes-test-suite test_ppu_basic_functionality
   ```

3. **Run Full Integration Tests**:
   ```bash
   cargo test -p rnes-test-suite --test ppu_integration_tests -- --ignored
   ```

4. **Run Specific Test Categories**:
   ```bash
   # Blargg's PPU tests
   cargo test -p rnes-test-suite test_blargg_ppu_suite -- --ignored
   
   # VBL NMI tests
   cargo test -p rnes-test-suite test_ppu_vbl_nmi -- --ignored
   
   # Full palette tests
   cargo test -p rnes-test-suite test_full_palette -- --ignored
   ```

### Using the PPU Test Runner

```rust
use rnes_test_suite::ppu_test_runner::{PpuTestRunner, PpuTestResult};

// Create a test runner
let mut runner = PpuTestRunner::new()
    .with_max_cycles(1000000)
    .with_max_frames(500)
    .with_frame_output_check(true);

// Load and run a test ROM
runner.load_rom(Path::new("path/to/test.nes"))?;

match runner.run_test()? {
    PpuTestResult::Completed { cycles, frames, final_frame, status } => {
        println!("Test completed: {} cycles, {} frames", cycles, frames);
        // Analyze frame output
    }
    PpuTestResult::Timeout { cycles, frames } => {
        println!("Test timed out: {} cycles, {} frames", cycles, frames);
    }
    PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
        println!("Infinite loop at PC=0x{:04X}", pc);
    }
}
```

## Test Results Analysis

### Frame Analysis

The framework includes tools for analyzing PPU output:

```rust
// Analyze palette output
fn analyze_palette_output(frame: &[Pixel]) {
    let mut color_counts = HashMap::new();
    for &pixel in frame {
        *color_counts.entry(pixel).or_insert(0) += 1;
    }
    
    println!("Unique colors: {}", color_counts.len());
    // Verify color count is within NES palette range (0-64)
}

// Analyze scrolling output
fn analyze_scrolling_output(frame: &[Pixel]) {
    let non_black_pixels = frame.iter()
        .filter(|&&p| p != Pixel::BLACK)
        .count();
    
    println!("Non-black pixels: {}", non_black_pixels);
    // Verify frame contains visible content
}
```

### Test Completion Detection

The framework detects test completion through various methods:

1. **VBlank Flag**: Many PPU tests set the VBlank flag when complete
2. **Frame Count**: Tests that render specific frame counts
3. **Memory Patterns**: Tests that write results to specific memory locations
4. **Infinite Loop Detection**: Heuristic detection of stuck execution

## M1 Implementation Focus

The current PPU testing framework is designed for **M1: PPU Background Rendering + NROM Mapper**:

### Supported Features
- ✅ Background rendering
- ✅ NROM mapper (Mapper 0)
- ✅ Basic PPU timing
- ✅ Palette system
- ✅ VBlank generation

### Not Yet Implemented
- ❌ Sprite rendering
- ❌ Advanced mappers (MMC1, MMC3, etc.)
- ❌ APU functionality
- ❌ Advanced PPU features

## Comparison with Reference Implementation

This testing framework is inspired by the [fogleman/nes](https://github.com/fogleman/nes) project, which provides:

- Comprehensive PPU test coverage
- Authoritative test ROMs
- Well-documented test procedures

Our implementation follows similar patterns while being adapted for the Rust-based RNES architecture.

## Future Enhancements

### Planned Features
- **Sprite Rendering Tests**: Once sprite rendering is implemented
- **Advanced Mapper Tests**: MMC1, MMC3, and other mapper support
- **Timing Accuracy Tests**: Precise PPU timing validation
- **Visual Regression Tests**: Automated visual output comparison
- **Performance Benchmarks**: PPU rendering performance measurement

### Integration with CI/CD
- Automated test ROM download in CI
- Visual output validation
- Performance regression detection
- Cross-platform testing

## Troubleshooting

### Common Issues

1. **Test ROMs Not Found**:
   ```bash
   ./scripts/download_ppu_test_roms.sh
   ```

2. **Unsupported Mapper**:
   - Current implementation supports only NROM (Mapper 0)
   - Other mappers will show "Unsupported mapper type" errors

3. **Blank Frame Output**:
   - May indicate PPU rendering issues
   - Check background rendering implementation
   - Verify palette system functionality

4. **Test Timeouts**:
   - Increase `max_cycles` and `max_frames` parameters
   - Check for infinite loops in PPU implementation
   - Verify test completion detection logic

### Debug Information

Enable debug output for detailed analysis:

```rust
// Access PPU state during testing
let ppu_state = runner.ppu_state();
let ppu_registers = runner.ppu_registers();
let frame_buffer = runner.frame_buffer();
```

## Contributing

When adding new PPU features:

1. **Add corresponding tests** to the integration test suite
2. **Update test ROMs** if new test categories are needed
3. **Document test procedures** in this guide
4. **Verify against reference implementations** like fogleman/nes

## References

- [NES PPU Documentation](http://wiki.nesdev.com/w/index.php/PPU)
- [Blargg's Test ROMs](http://blargg.8bitalley.com/nes-tests/)
- [fogleman/nes Reference Implementation](https://github.com/fogleman/nes)
- [NES Test ROMs Repository](https://github.com/christopherpow/nes-test-roms)
