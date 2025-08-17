#!/bin/bash

# Download M2 (Sprite Layer & Input) test ROMs
# This script downloads test ROMs specifically for testing M2 functionality

set -e

echo "Downloading M2 test ROMs..."

# Base directory for test ROMs
TEST_ROMS_DIR="tests/roms/nes-test-roms"

# Create directory if it doesn't exist
mkdir -p "$TEST_ROMS_DIR"

# Change to test ROMs directory
cd "$TEST_ROMS_DIR"

# Clone the nes-test-roms repository if it doesn't exist
if [ ! -d ".git" ]; then
    echo "Cloning nes-test-roms repository..."
    git clone https://github.com/christopherpow/nes-test-roms.git .
fi

# Update the repository
echo "Updating nes-test-roms repository..."
git pull origin master

# Verify that M2-specific test ROMs are present
echo "Verifying M2 test ROMs..."

M2_TESTS=(
    "sprite_hit_tests_2005.10.05"
    "sprite_overflow_tests"
    "oam_stress"
    "oam_read"
    "sprdma_and_dmc_dma"
)

for test_dir in "${M2_TESTS[@]}"; do
    if [ -d "$test_dir" ]; then
        echo "✅ Found $test_dir"
    else
        echo "❌ Missing $test_dir"
    fi
done

echo ""
echo "M2 test ROMs download complete!"
echo ""
echo "Available M2 test suites:"
echo "  - sprite_hit_tests_2005.10.05: Tests sprite 0 hit detection"
echo "  - sprite_overflow_tests: Tests sprite overflow flag"
echo "  - oam_stress: Tests OAM read/write operations"
echo "  - oam_read: Tests OAM read functionality"
echo "  - sprdma_and_dmc_dma: Tests sprite DMA operations"
echo ""
echo "To run M2 tests:"
echo "  cargo test -p rnes-test-suite --test m2_integration_tests -- --ignored"
