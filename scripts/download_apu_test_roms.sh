#!/bin/bash

# Script to download authoritative NES APU test ROMs
# For testing M3: APU Audio System functionality

set -e

TEST_ROMS_DIR="tests/roms"
APU_TEST_DIR="$TEST_ROMS_DIR/apu-tests"

echo "🚀 Starting NES APU test ROM download..."
echo "📋 Testing M3: APU Audio System"

# Create test ROM directory
mkdir -p "$TEST_ROMS_DIR"
mkdir -p "$APU_TEST_DIR"

echo "📥 Downloading APU test ROMs..."

# Download from various authoritative sources
cd "$APU_TEST_DIR"

# 1. Blargg's APU tests (most authoritative)
if [ ! -d "blargg_apu_tests" ]; then
    echo "📥 Downloading Blargg's APU tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/blargg_apu_2005.07.30 blargg_apu_tests
    rm -rf temp_nes_tests
fi

# 2. APU reset tests
if [ ! -d "apu_reset" ]; then
    echo "📥 Downloading APU reset tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/apu_reset apu_reset
    rm -rf temp_nes_tests
fi

# 3. APU test (general)
if [ ! -d "apu_test" ]; then
    echo "📥 Downloading APU test..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/apu_test apu_test
    rm -rf temp_nes_tests
fi

# 4. APU mixer tests
if [ ! -d "apu_mixer" ]; then
    echo "📥 Downloading APU mixer tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/apu_mixer apu_mixer
    rm -rf temp_nes_tests
fi

# 5. PAL APU tests
if [ ! -d "pal_apu_tests" ]; then
    echo "📥 Downloading PAL APU tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/pal_apu_tests pal_apu_tests
    rm -rf temp_nes_tests
fi

cd - > /dev/null

echo "✅ APU test ROM download completed!"
echo ""
echo "📋 Available APU test ROMs for M3:"
echo "  - blargg_apu_tests/          # Blargg's comprehensive APU tests"
echo "  - apu_reset/                 # APU reset behavior tests"
echo "  - apu_test/                  # General APU functionality tests"
echo "  - apu_mixer/                 # APU audio mixing tests"
echo "  - pal_apu_tests/             # PAL APU timing tests"
echo ""
echo "🎯 M3 Test Focus Areas:"
echo "  ✓ 5 audio channels (Pulse 1&2, Triangle, Noise, DMC)"
echo "  ✓ Audio mixing and output"
echo "  ✓ Timing synchronization with CPU/PPU"
echo "  ✓ Volume and envelope control"
echo "  ✓ Frame counter and IRQ handling"
echo ""
echo "💡 Usage:"
echo "  cargo run --bin apu_integration_test"
echo "  cargo test -p rnes-test-suite --test apu_integration_tests -- --ignored"
