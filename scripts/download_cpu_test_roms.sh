#!/bin/bash

# Script to download NES CPU test ROMs
# Based on https://github.com/christopherpow/nes-test-roms

set -e

TEST_ROMS_DIR="tests/roms"
REPO_URL="https://github.com/christopherpow/nes-test-roms"

echo "🚀 Starting NES CPU test ROM download..."

# Create test ROM directory
mkdir -p "$TEST_ROMS_DIR"

# Check if already exists
if [ -d "$TEST_ROMS_DIR/nes-test-roms" ]; then
    echo "📁 Test ROM directory already exists, skipping download"
    exit 0
fi

# Download repository
echo "📥 Cloning test ROM repository..."
git clone "$REPO_URL" "$TEST_ROMS_DIR/nes-test-roms"

echo "✅ CPU test ROM download completed!"
echo ""
echo "📋 Available CPU test ROMs:"
echo "  - blargg_nes_cpu_test5/     # 6502 CPU test suite"
echo "  - cpu_dummy_reads/          # CPU dummy reads test"
echo "  - cpu_dummy_writes/         # CPU dummy writes test"
echo "  - branch_timing_tests/      # Branch instruction timing tests"
echo ""
echo "📋 Additional test ROMs (also downloaded):"
echo "  - apu_test/                 # Audio Processing Unit tests"
echo "  - blargg_ppu_tests/         # Picture Processing Unit tests"
echo ""
echo "💡 CPU Testing Usage:"
echo "  cargo test -p rnes-test-suite --test cpu_integration_tests"
echo "  cargo test -p rnes-test-suite --test cpu_integration_tests -- --ignored"
