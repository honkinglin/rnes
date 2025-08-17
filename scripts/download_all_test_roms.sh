#!/bin/bash

# Script to download all NES test ROMs (CPU, PPU, APU, etc.)
# This script downloads both CPU and PPU test ROMs

set -e

echo "🚀 Starting comprehensive NES test ROM download..."
echo "=================================================="

# Download CPU test ROMs
echo ""
echo "📥 Downloading CPU test ROMs..."
./scripts/download_cpu_test_roms.sh

# Download PPU test ROMs
echo ""
echo "📥 Downloading PPU test ROMs..."
./scripts/download_ppu_test_roms.sh

echo ""
echo "🎉 All test ROMs download completed!"
echo ""
echo "📋 Available test ROMs:"
echo ""
echo "🔧 CPU Tests:"
echo "  - blargg_nes_cpu_test5/     # 6502 CPU test suite"
echo "  - cpu_dummy_reads/          # CPU dummy reads test"
echo "  - cpu_dummy_writes/         # CPU dummy writes test"
echo "  - branch_timing_tests/      # Branch instruction timing tests"
echo ""
echo "🎨 PPU Tests:"
echo "  - blargg_ppu_tests/         # Blargg's comprehensive PPU tests"
echo "  - ppu_vbl_nmi/              # VBlank and NMI timing tests"
echo "  - ppu_read_buffer/          # PPU read buffer behavior tests"
echo "  - full_palette/             # Full palette rendering tests"
echo "  - scrolltest/               # Background scrolling tests"
echo "  - nrom368/                  # NROM mapper specific tests"
echo ""
echo "🔊 Audio Tests:"
echo "  - apu_test/                 # Audio Processing Unit tests"
echo ""
echo "💡 Usage:"
echo "  # Run CPU tests"
echo "  cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored"
echo ""
echo "  # Run PPU tests"
echo "  cargo test -p rnes-test-suite --test ppu_integration_tests -- --ignored"
echo ""
echo "  # Run all tests"
echo "  cargo test -p rnes-test-suite"
