#!/bin/bash

# Download M4 test ROMs for Common Mappers testing
# This script downloads test ROMs for MMC1, UxROM, CNROM, and AOROM mappers

set -e

echo "Downloading M4 test ROMs for Common Mappers..."
echo "=============================================="

# Create test ROMs directory
TEST_ROMS_DIR="tests/roms/nes-test-roms"
mkdir -p "$TEST_ROMS_DIR"

# Create mapper-specific test directories
mkdir -p "$TEST_ROMS_DIR/mmc1_tests"
mkdir -p "$TEST_ROMS_DIR/uxrom_tests"
mkdir -p "$TEST_ROMS_DIR/cnrom_tests"
mkdir -p "$TEST_ROMS_DIR/aorom_tests"

echo "✓ Created test directories"

# Note: In a real implementation, you would download actual test ROMs here
# For now, we'll create placeholder files and note that real test ROMs
# would need to be obtained from legitimate sources

echo ""
echo "⚠️  Note: This script creates placeholder files for testing."
echo "   For comprehensive mapper testing, you would need to obtain"
echo "   actual test ROMs from legitimate sources such as:"
echo "   - nesdev.com test ROMs"
echo "   - Homebrew ROMs with specific mapper implementations"
echo "   - Commercial games that use these mappers"
echo ""

# Create placeholder test ROMs for each mapper
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

# Create placeholder ROMs for each mapper
create_placeholder_rom 1 "MMC1" "$TEST_ROMS_DIR/mmc1_tests"
create_placeholder_rom 2 "UxROM" "$TEST_ROMS_DIR/uxrom_tests"
create_placeholder_rom 3 "CNROM" "$TEST_ROMS_DIR/cnrom_tests"
create_placeholder_rom 7 "AOROM" "$TEST_ROMS_DIR/aorom_tests"

echo ""
echo "✓ M4 test ROMs setup completed!"
echo ""
echo "To run M4 integration tests:"
echo "  cargo test -p rnes-test-suite --test m4_integration_tests -- --ignored"
echo ""
echo "To run the M4 demo:"
echo "  cargo run --example m4_demo"
echo ""
echo "Note: The integration tests are marked with #[ignore] because"
echo "they require actual test ROMs. Remove the #[ignore] attribute"
echo "once you have obtained legitimate test ROMs."
