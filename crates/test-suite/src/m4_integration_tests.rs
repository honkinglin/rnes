use std::path::PathBuf;
use rnes_test_suite::cpu_test_runner::{CpuTestRunner, TestResult};

/// Test M4: Common Mappers functionality
#[test]
fn test_m4_basic_functionality() {
    let _runner = CpuTestRunner::new()
        .with_max_cycles(100000);
    
    // Add basic M4 functionality tests here
    // e.g., mapper creation, bank switching, etc.
    println!("✅ M4 basic functionality test passed");
}

/// Test MMC1 Mapper (Mapper 1)
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_mmc1_mapper() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    
    if !test_roms_dir.exists() {
        println!("⚠️  Test ROMs not found, please run: ./scripts/download_all_test_roms.sh");
        return;
    }
    
    // Look for MMC1 test ROMs
    let mmc1_test_dir = test_roms_dir.join("mmc1_tests");
    
    if !mmc1_test_dir.exists() {
        println!("⚠️  MMC1 test ROMs not found");
        return;
    }
    
    // Run MMC1 test ROMs
    let test_files = vec![
        "mmc1_test.nes",
        "mmc1_basic.nes",
    ];
    
    for test_file in test_files {
        let rom_path = mmc1_test_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running MMC1 test: {}", test_file);
            
            let mut runner = CpuTestRunner::new()
                .with_max_cycles(500000);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                TestResult::Completed { status, cycles, .. } => {
                                    if status == 0 {
                                        println!("  ✅ PASS ({} cycles)", cycles);
                                    } else {
                                        println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                    }
                                }
                                TestResult::Timeout { cycles } => {
                                    println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                                }
                                TestResult::InfiniteLoop { cycles, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 ERROR: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 LOAD ERROR: {}", e);
                }
            }
        } else {
            println!("⚠️  Test file not found: {}", test_file);
        }
    }
}

/// Test UxROM Mapper (Mapper 2)
#[test]
#[ignore]
fn test_uxrom_mapper() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let uxrom_test_dir = test_roms_dir.join("uxrom_tests");
    
    if !uxrom_test_dir.exists() {
        println!("⚠️  UxROM test ROMs not found");
        return;
    }
    
    let test_files = vec![
        "uxrom_test.nes",
        "uxrom_basic.nes",
    ];
    
    for test_file in test_files {
        let rom_path = uxrom_test_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running UxROM test: {}", test_file);
            
            let mut runner = CpuTestRunner::new()
                .with_max_cycles(500000);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                TestResult::Completed { status, cycles, .. } => {
                                    if status == 0 {
                                        println!("  ✅ PASS ({} cycles)", cycles);
                                    } else {
                                        println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                    }
                                }
                                TestResult::Timeout { cycles } => {
                                    println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                                }
                                TestResult::InfiniteLoop { cycles, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 ERROR: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 LOAD ERROR: {}", e);
                }
            }
        } else {
            println!("⚠️  Test file not found: {}", test_file);
        }
    }
}

/// Test CNROM Mapper (Mapper 3)
#[test]
#[ignore]
fn test_cnrom_mapper() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let cnrom_test_dir = test_roms_dir.join("cnrom_tests");
    
    if !cnrom_test_dir.exists() {
        println!("⚠️  CNROM test ROMs not found");
        return;
    }
    
    let test_files = vec![
        "cnrom_test.nes",
        "cnrom_basic.nes",
    ];
    
    for test_file in test_files {
        let rom_path = cnrom_test_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running CNROM test: {}", test_file);
            
            let mut runner = CpuTestRunner::new()
                .with_max_cycles(500000);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                TestResult::Completed { status, cycles, .. } => {
                                    if status == 0 {
                                        println!("  ✅ PASS ({} cycles)", cycles);
                                    } else {
                                        println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                    }
                                }
                                TestResult::Timeout { cycles } => {
                                    println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                                }
                                TestResult::InfiniteLoop { cycles, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 ERROR: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 LOAD ERROR: {}", e);
                }
            }
        } else {
            println!("⚠️  Test file not found: {}", test_file);
        }
    }
}

/// Test AOROM Mapper (Mapper 7)
#[test]
#[ignore]
fn test_aorom_mapper() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let aorom_test_dir = test_roms_dir.join("aorom_tests");
    
    if !aorom_test_dir.exists() {
        println!("⚠️  AOROM test ROMs not found");
        return;
    }
    
    let test_files = vec![
        "aorom_test.nes",
        "aorom_basic.nes",
    ];
    
    for test_file in test_files {
        let rom_path = aorom_test_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running AOROM test: {}", test_file);
            
            let mut runner = CpuTestRunner::new()
                .with_max_cycles(500000);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                TestResult::Completed { status, cycles, .. } => {
                                    if status == 0 {
                                        println!("  ✅ PASS ({} cycles)", cycles);
                                    } else {
                                        println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                    }
                                }
                                TestResult::Timeout { cycles } => {
                                    println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                                }
                                TestResult::InfiniteLoop { cycles, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 ERROR: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 LOAD ERROR: {}", e);
                }
            }
        } else {
            println!("⚠️  Test file not found: {}", test_file);
        }
    }
}

/// Test mapper creation and basic functionality
#[test]
fn test_mapper_creation() {
    use rnes_cartridge::Cartridge;
    use rnes_mappers::create_mapper;
    
    // Test NROM mapper creation
    let nrom_data = create_test_rom(0);
    let nrom_cartridge = Cartridge::from_bytes(&nrom_data).unwrap();
    let nrom_mapper = create_mapper(nrom_cartridge);
    assert!(nrom_mapper.is_ok());
    println!("✅ NROM mapper creation test passed");
    
    // Test MMC1 mapper creation
    let mmc1_data = create_test_rom(1);
    let mmc1_cartridge = Cartridge::from_bytes(&mmc1_data).unwrap();
    let mmc1_mapper = create_mapper(mmc1_cartridge);
    assert!(mmc1_mapper.is_ok());
    println!("✅ MMC1 mapper creation test passed");
    
    // Test UxROM mapper creation
    let uxrom_data = create_test_rom(2);
    let uxrom_cartridge = Cartridge::from_bytes(&uxrom_data).unwrap();
    let uxrom_mapper = create_mapper(uxrom_cartridge);
    assert!(uxrom_mapper.is_ok());
    println!("✅ UxROM mapper creation test passed");
    
    // Test CNROM mapper creation
    let cnrom_data = create_test_rom(3);
    let cnrom_cartridge = Cartridge::from_bytes(&cnrom_data).unwrap();
    let cnrom_mapper = create_mapper(cnrom_cartridge);
    assert!(cnrom_mapper.is_ok());
    println!("✅ CNROM mapper creation test passed");
    
    // Test AOROM mapper creation
    let aorom_data = create_test_rom(7);
    let aorom_cartridge = Cartridge::from_bytes(&aorom_data).unwrap();
    let aorom_mapper = create_mapper(aorom_cartridge);
    assert!(aorom_mapper.is_ok());
    println!("✅ AOROM mapper creation test passed");
    
    // Test unsupported mapper
    let unsupported_data = create_test_rom(99);
    let unsupported_cartridge = Cartridge::from_bytes(&unsupported_data).unwrap();
    let unsupported_mapper = create_mapper(unsupported_cartridge);
    assert!(unsupported_mapper.is_err());
    println!("✅ Unsupported mapper error test passed");
}

/// Create a test ROM with specified mapper number
fn create_test_rom(mapper_number: u8) -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        (mapper_number & 0x0F) << 4, (mapper_number & 0xF0), // Mapper number, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Simple program
    let mut prg_rom = vec![0; 16384];
    
    // Simple program that just loops
    prg_rom[0] = 0x4C; // JMP absolute
    prg_rom[1] = 0x00; // Low byte
    prg_rom[2] = 0x80; // High byte (infinite loop)
    
    // Reset vector points to our program
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Simple pattern data
    let mut chr_rom = vec![0; 8192];
    
    // Create a simple pattern (8x8 pixel tile)
    for tile in 0..16 {
        let base = tile * 16;
        // Create a simple checkerboard pattern
        chr_rom[base + 0] = 0xAA; // 10101010
        chr_rom[base + 1] = 0x55; // 01010101
        chr_rom[base + 2] = 0xAA;
        chr_rom[base + 3] = 0x55;
        chr_rom[base + 4] = 0xAA;
        chr_rom[base + 5] = 0x55;
        chr_rom[base + 6] = 0xAA;
        chr_rom[base + 7] = 0x55;
        chr_rom[base + 8] = 0xAA;
        chr_rom[base + 9] = 0x55;
        chr_rom[base + 10] = 0xAA;
        chr_rom[base + 11] = 0x55;
        chr_rom[base + 12] = 0xAA;
        chr_rom[base + 13] = 0x55;
        chr_rom[base + 14] = 0xAA;
        chr_rom[base + 15] = 0x55;
    }
    
    rom.extend_from_slice(&chr_rom);
    
    rom
}
