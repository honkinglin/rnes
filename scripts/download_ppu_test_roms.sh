#!/bin/bash

# Script to download authoritative NES PPU test ROMs
# For testing M1: PPU Background Rendering + NROM functionality

set -e

TEST_ROMS_DIR="tests/roms"
PPU_TEST_DIR="$TEST_ROMS_DIR/ppu-tests"

echo "🚀 Starting NES PPU test ROM download..."
echo "📋 Testing M1: PPU Background Rendering + NROM"

# Create test ROM directory
mkdir -p "$TEST_ROMS_DIR"
mkdir -p "$PPU_TEST_DIR"

echo "📥 Downloading PPU test ROMs..."

# Download from various authoritative sources
cd "$PPU_TEST_DIR"

# 1. Blargg's PPU tests (most authoritative)
if [ ! -d "blargg_ppu_tests" ]; then
    echo "📥 Downloading Blargg's PPU tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/blargg_ppu_tests_2005.09.15b blargg_ppu_tests
    rm -rf temp_nes_tests
fi

# 2. PPU VBL NMI tests
if [ ! -d "ppu_vbl_nmi" ]; then
    echo "📥 Downloading PPU VBL NMI tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/ppu_vbl_nmi ppu_vbl_nmi
    rm -rf temp_nes_tests
fi

# 3. PPU read buffer tests
if [ ! -d "ppu_read_buffer" ]; then
    echo "📥 Downloading PPU read buffer tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/ppu_read_buffer ppu_read_buffer
    rm -rf temp_nes_tests
fi

# 4. Full palette tests
if [ ! -d "full_palette" ]; then
    echo "📥 Downloading full palette tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/full_palette full_palette
    rm -rf temp_nes_tests
fi

# 5. Scroll tests
if [ ! -d "scrolltest" ]; then
    echo "📥 Downloading scroll tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/scrolltest scrolltest
    rm -rf temp_nes_tests
fi

# 6. NROM 368 tests
if [ ! -d "nrom368" ]; then
    echo "📥 Downloading NROM 368 tests..."
    git clone https://github.com/christopherpow/nes-test-roms.git temp_nes_tests
    cp -r temp_nes_tests/nrom368 nrom368
    rm -rf temp_nes_tests
fi

cd - > /dev/null

echo "✅ PPU test ROM download completed!"
echo ""
echo "📋 Available PPU test ROMs for M1:"
echo "  - blargg_ppu_tests/          # Blargg's comprehensive PPU tests"
echo "  - ppu_vbl_nmi/              # VBlank and NMI timing tests"
echo "  - ppu_read_buffer/          # PPU read buffer behavior tests"
echo "  - full_palette/             # Full palette rendering tests"
echo "  - scrolltest/               # Background scrolling tests"
echo "  - nrom368/                  # NROM mapper specific tests"
echo ""
echo "🎯 M1 Test Focus Areas:"
echo "  ✓ PPU timing model"
echo "  ✓ Background rendering"
echo "  ✓ NROM Mapper"
echo "  ✓ Palette system"
echo ""
echo "💡 Usage:"
echo "  cargo run --bin rom_test"
echo "  cargo run --bin ppu_integration_test"
