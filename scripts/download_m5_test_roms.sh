#!/bin/bash

# Download M5 test ROMs for MMC3 mapper testing
# This script downloads test ROMs for MMC3 mapper with scanline IRQ

set -e

echo "Downloading M5 test ROMs for MMC3 Mapper..."
echo "=========================================="

# Create test ROMs directory
TEST_ROMS_DIR="tests/roms/nes-test-roms"
mkdir -p "$TEST_ROMS_DIR"

# Create mapper-specific test directories
mkdir -p "$TEST_ROMS_DIR/mmc3_tests"

echo "✓ Created test directories"

# Note: In a real implementation, you would download actual test ROMs here
# For now, we'll create placeholder files and note that real test ROMs
# would need to be obtained from legitimate sources

echo ""
echo "⚠️  Note: This script creates placeholder files for testing."
echo "   For comprehensive MMC3 mapper testing, you would need to obtain"
echo "   actual test ROMs from legitimate sources such as:"
echo "   - nesdev.com test ROMs"
echo "   - Homebrew ROMs with MMC3 implementations"
echo "   - Commercial games that use MMC3 mapper"
echo ""

# Create placeholder test ROMs for MMC3 mapper
create_placeholder_rom() {
    local mapper_num=$1
    local mapper_name=$2
    local output_dir=$3
    
    echo "Creating placeholder ROM for $mapper_name (Mapper $mapper_num)..."
    
    # Create a simple test ROM with the specified mapper
    cat > "$output_dir/$(echo $mapper_name | tr '[:upper:]' '[:lower:]')_test.nes" << EOF
# Placeholder for $mapper_name test ROM
# This would be a real .nes file with mapper $mapper_num
# For testing, you would need to obtain actual test ROMs
EOF
    
    echo "✓ Created placeholder for $mapper_name"
}

# Create placeholder ROMs for MMC3 mapper
create_placeholder_rom 4 "MMC3" "$TEST_ROMS_DIR/mmc3_tests"

echo ""
echo "✓ M5 test ROMs setup completed!"
echo ""
echo "To run M5 integration tests:"
echo "  cargo test -p rnes-test-suite --test m5_integration_tests -- --ignored"
echo ""
echo "To run the M5 demo:"
echo "  cargo run --example m5_demo"
echo ""
echo "Note: The integration tests are marked with #[ignore] because"
echo "they require actual test ROMs. Remove the #[ignore] attribute"
echo "once you have obtained legitimate test ROMs."
