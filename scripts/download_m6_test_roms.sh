#!/bin/bash

# Download M6 test ROMs
# This script downloads ROMs for testing M6 features (configuration, debugger, save states)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ROMS_DIR="$PROJECT_ROOT/tests/roms"

echo "🎮 Downloading M6 test ROMs..."
echo "================================"

# Create ROMs directory if it doesn't exist
mkdir -p "$ROMS_DIR"

# Check if nes-test-roms repository exists
if [ ! -d "$ROMS_DIR/nes-test-roms" ]; then
    echo "📥 Cloning nes-test-roms repository..."
    cd "$ROMS_DIR"
    git clone https://github.com/christopherpow/nes-test-roms.git
    cd "$PROJECT_ROOT"
fi

echo "📥 Copying M6 test ROMs..."

# Copy test ROMs for M6 features
# These are simple, reliable test ROMs that work well with debugging and save states

# 1. CPU reset test ROM - good for debugging CPU state
if [ ! -f "$ROMS_DIR/m6_cpu_reset.nes" ]; then
    echo "Copying CPU reset test ROM..."
    cp "$ROMS_DIR/nes-test-roms/cpu_reset/registers.nes" "$ROMS_DIR/m6_cpu_reset.nes"
fi

# 2. CPU reset RAM test ROM - good for memory debugging
if [ ! -f "$ROMS_DIR/m6_ram_reset.nes" ]; then
    echo "Copying RAM reset test ROM..."
    cp "$ROMS_DIR/nes-test-roms/cpu_reset/ram_after_reset.nes" "$ROMS_DIR/m6_ram_reset.nes"
fi

# 3. Instruction misc test ROM - good for step-by-step debugging
if [ ! -f "$ROMS_DIR/m6_instr_misc.nes" ]; then
    echo "Copying instruction misc test ROM..."
    cp "$ROMS_DIR/nes-test-roms/instr_misc/instr_misc.nes" "$ROMS_DIR/m6_instr_misc.nes"
fi

# 4. CPU timing test ROM - good for timing debugging
if [ ! -f "$ROMS_DIR/m6_cpu_timing.nes" ]; then
    echo "Copying CPU timing test ROM..."
    cp "$ROMS_DIR/nes-test-roms/cpu_timing_test6/cpu_timing_test.nes" "$ROMS_DIR/m6_cpu_timing.nes"
fi

# 5. Branch timing test ROM - good for branch debugging
if [ ! -f "$ROMS_DIR/m6_branch_timing.nes" ]; then
    echo "Copying branch timing test ROM..."
    cp "$ROMS_DIR/nes-test-roms/branch_timing_tests/1.Branch_Basics.nes" "$ROMS_DIR/m6_branch_timing.nes"
fi

# 6. Simple PPU test ROM - good for PPU debugging
if [ ! -f "$ROMS_DIR/m6_ppu_palette.nes" ]; then
    echo "Copying PPU palette test ROM..."
    cp "$ROMS_DIR/nes-test-roms/blargg_ppu_tests_2005.09.15b/palette_ram.nes" "$ROMS_DIR/m6_ppu_palette.nes"
fi

echo ""
echo "✅ M6 test ROMs setup complete!"
echo ""
echo "Test ROMs available:"
echo "  - m6_cpu_reset.nes (CPU register reset test - good for debugging)"
echo "  - m6_ram_reset.nes (RAM reset test - good for memory debugging)"
echo "  - m6_instr_misc.nes (Instruction misc test - good for step-by-step debugging)"
echo "  - m6_cpu_timing.nes (CPU timing test - good for timing debugging)"
echo "  - m6_branch_timing.nes (Branch timing test - good for branch debugging)"
echo "  - m6_ppu_palette.nes (PPU palette test - good for PPU debugging)"
echo ""
echo "To run M6 tests:"
echo "  cargo test -p rnes-test-suite --test m6_integration_tests"
echo ""
echo "To run M6 tests with ROMs:"
echo "  cargo test -p rnes-test-suite --test m6_integration_tests -- --ignored"
echo ""
echo "To run M6 demo:"
echo "  cargo run --bin m6_demo"
