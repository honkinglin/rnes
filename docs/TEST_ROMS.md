# Test ROMs

This document explains how test ROMs are used in the RNES project and why they are not included in the repository.

## Overview

RNES uses test ROMs to validate the accuracy of the 6502 CPU implementation. These ROMs are essential for ensuring the emulator correctly implements all instructions, addressing modes, and timing behaviors.

## Test ROM Sources

The test ROMs are sourced from the [nes-test-roms](https://github.com/christopherpow/nes-test-roms) repository, which contains a comprehensive collection of test ROMs for NES emulator development.

## Why Test ROMs Are Not Included

### 1. Copyright Considerations
- Test ROMs may be subject to copyright protection
- Including them in a public repository could raise legal issues
- Even if the ROMs are freely available, redistribution rights may be unclear

### 2. Repository Size
- Test ROMs are binary files that can be quite large
- Including them would significantly increase repository size
- This would slow down cloning and increase storage requirements

### 3. Maintenance
- Binary files don't benefit from Git's version control features
- Updates to test ROMs would bloat repository history
- Separate management is more efficient

## How to Get Test ROMs

### Automatic Download
The project includes a script to automatically download test ROMs:

```bash
./scripts/download_test_roms.sh
```

This script:
- Clones the nes-test-roms repository
- Places ROMs in `tests/roms/nes-test-roms/`
- Skips download if ROMs already exist

### Manual Download
If you prefer to download manually:

1. Visit [nes-test-roms](https://github.com/christopherpow/nes-test-roms)
2. Clone or download the repository
3. Place the contents in `tests/roms/nes-test-roms/`

## Available Test ROMs

After downloading, you'll have access to:

- **blargg_nes_cpu_test5/**: Complete 6502 CPU test suite
- **cpu_dummy_reads/**: CPU dummy read behavior tests
- **branch_timing_tests/**: Branch instruction timing tests
- **apu_test/**: Audio Processing Unit tests
- **blargg_ppu_tests/**: Picture Processing Unit tests
- And many more...

## Running Tests

### Basic Tests (No ROMs Required)
```bash
cargo test -p rnes-test-suite
```

### Integration Tests (Requires ROMs)
```bash
# Download ROMs first
./scripts/download_test_roms.sh

# Run all integration tests
cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored

# Run specific tests
cargo test -p rnes-test-suite --test cpu_integration_tests test_cpu_basic_functionality
```

## Git Ignore Configuration

The test ROMs directory is automatically ignored by Git:

```
# .gitignore
/tests/roms/nes-test-roms/
```

This ensures that:
- ROMs are never accidentally committed
- Repository size remains manageable
- Copyright concerns are addressed

## Contributing

When contributing to RNES:

1. **Don't commit test ROMs** - They're automatically ignored
2. **Update the download script** if ROM sources change
3. **Document new test ROMs** in this file
4. **Ensure tests work** with the current ROM set

## Troubleshooting

### ROMs Not Found
If tests report "Test ROMs not found":
```bash
./scripts/download_test_roms.sh
```

### Download Fails
If the download script fails:
1. Check your internet connection
2. Verify the nes-test-roms repository is accessible
3. Try manual download as described above

### Tests Still Fail
If tests fail even with ROMs:
1. Check ROM file permissions
2. Verify ROM file integrity
3. Review test output for specific errors
4. Check CPU implementation for issues

## Legal Notice

The test ROMs used in this project are for educational and testing purposes only. Please respect the original authors' rights and the terms under which these ROMs are distributed.
