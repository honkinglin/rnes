# NES Emulator Testing Guide

## Overview

This project contains a comprehensive test suite for validating the correct implementation of various NES emulator components.

## Test Suite Structure

The test suite is located in the `crates/test-suite/` directory and includes the following components:

- **CPU Integration Tests** (`cpu_integration_tests.rs`): Tests for 6502 CPU functionality
- **CPU Test Runner** (`cpu_test_runner.rs`): Infrastructure for running test ROMs

## Running Tests

### Basic Tests

Run all basic tests (excluding ROM-dependent tests):

```bash
cargo test -p rnes-test-suite
```

### CPU Integration Tests

Run CPU integration tests:

```bash
# Run basic functionality tests
cargo test -p rnes-test-suite --test cpu_integration_tests

# Run all tests (including ignored ROM tests)
cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored

# Run specific test
cargo test -p rnes-test-suite --test cpu_integration_tests test_cpu_basic_functionality

# Show test output
cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored --nocapture
```

### Test ROMs

#### Downloading Test ROMs

The project includes a script to download various NES test ROMs:

```bash
./scripts/download_test_roms.sh
```

This will download the following test suites:

- **Blargg CPU Test Suite**: 6502 CPU functionality tests
- **CPU Dummy Reads Test**: Tests CPU dummy read behavior
- **Branch Timing Tests**: Tests branch instruction timing
- **APU Tests**: Audio Processing Unit tests
- **PPU Tests**: Picture Processing Unit tests

#### Available Test ROMs

After downloading, the following test ROMs will be available for testing:

```
tests/roms/nes-test-roms/
├── blargg_nes_cpu_test5/     # 6502 CPU test suite
│   ├── cpu.nes
│   └── official.nes
├── cpu_dummy_reads/          # CPU dummy reads test
│   └── cpu_dummy_reads.nes
├── branch_timing_tests/      # Branch instruction timing tests
│   ├── 1.Branch_Basics.nes
│   ├── 2.Backward_Branch.nes
│   └── 3.Forward_Branch.nes
└── ...                       # Other test suites
```

## Test Result Interpretation

The test runner reports the following types of results:

- **PASS**: Test completed successfully with status code 0
- **FAIL**: Test failed with non-zero status code
- **TIMEOUT**: Test timed out (exceeded maximum cycles)
- **INFINITE LOOP**: Infinite loop detected
- **LOAD ERROR**: ROM loading failed

## Debugging Tests

If tests fail, you can:

1. **Increase maximum cycles**: Modify the `with_max_cycles()` parameter in tests
2. **Check ROM paths**: Ensure test ROMs are properly downloaded
3. **View detailed output**: Use the `--nocapture` flag to see test output
4. **Run individual tests**: Use the `--test` parameter to run specific tests

## Adding New Tests

To add new tests:

1. Add new test functions in `crates/test-suite/src/cpu_integration_tests.rs`
2. Mark tests with `#[test]` and `#[ignore]` attributes
3. Use `CpuTestRunner` to run test ROMs

Example:

```rust
#[test]
#[ignore]
fn test_new_feature() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let rom_path = test_roms_dir.join("your_test/test.nes");
    
    if rom_path.exists() {
        let mut runner = CpuTestRunner::new()
            .with_max_cycles(100000);
        
        match runner.load_rom(&rom_path) {
            Ok(_) => {
                match runner.run_test() {
                    Ok(result) => {
                        // Handle test results
                    }
                    Err(e) => println!("ERROR: {}", e),
                }
            }
            Err(e) => println!("LOAD ERROR: {}", e),
        }
    }
}
```

## Troubleshooting

### Common Issues

1. **"no test target named" error**
   - Ensure you're using the correct command: `cargo test -p rnes-test-suite --test cpu_integration_tests`

2. **Test ROMs not found**
   - Run `./scripts/download_test_roms.sh` to download test ROMs

3. **Tests stuck in infinite loops**
   - This usually indicates issues with CPU implementation that need debugging

4. **Compilation errors**
   - Ensure all dependent crates are properly implemented
   - Check import statements

### Getting Help

If you encounter issues:

1. Check error messages in test output
2. Review relevant CPU implementation code
3. Refer to NES technical documentation
4. Check test ROM documentation
