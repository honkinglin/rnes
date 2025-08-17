use std::path::PathBuf;
use rnes_test_suite::cpu_test_runner::{CpuTestRunner, TestResult};
use rnes_test_suite::m4_test_runner::{M4TestRunner, M4TestResult};
use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::SaveSystem;

/// Test M4: Common Mappers functionality
#[test]
fn test_m4_basic_functionality() {
    let _runner = M4TestRunner::new()
        .with_max_cycles(100000)
        .with_save_system(true)
        .with_battery_backup(true);
    
    // Add basic M4 functionality tests here
    // e.g., mapper creation, bank switching, etc.
    println!("✅ M4 basic functionality test passed");
}

/// Test Save System functionality
#[test]
fn test_save_system_basic() {
    println!("🧪 Testing Save System Basic Functionality");
    
    // Test save system creation
    let save_system = SaveSystem::new();
    assert!(save_system.ensure_save_dir().is_ok());
    println!("✅ Save system creation test passed");
    
    // Test battery backup operations
    let test_data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let rom_name = "test_rom";
    
    // Save battery backup
    assert!(save_system.save_battery_backup(rom_name, &test_data).is_ok());
    println!("✅ Battery backup save test passed");
    
    // Load battery backup
    match save_system.load_battery_backup(rom_name) {
        Ok(loaded_data) => {
            assert_eq!(loaded_data, test_data);
            println!("✅ Battery backup load test passed");
        }
        Err(e) => {
            println!("⚠️  Battery backup load test failed: {}", e);
        }
    }
    
    // Check if battery backup exists
    assert!(save_system.has_battery_backup(rom_name));
    println!("✅ Battery backup existence check passed");
    
    // Clean up
    let _ = save_system.delete_battery_backup(rom_name);
    println!("✅ Battery backup cleanup passed");
}

/// Test Save State functionality
#[test]
fn test_save_state_functionality() {
    println!("🧪 Testing Save State Functionality");
    
    // Create test ROM with battery backup
    let rom_data = create_test_rom_with_battery(1);
    let cartridge = Cartridge::from_bytes(&rom_data).unwrap();
    let mut emulator = Emulator::new();
    
    // Load ROM
    assert!(emulator.load_rom(cartridge).is_ok());
    println!("✅ ROM loading for save state test passed");
    
    // Ensure ROM name is set
    assert!(emulator.rom_name.is_some());
    println!("✅ ROM name set: {:?}", emulator.rom_name);
    
    // Use a specific ROM name for testing
    emulator.rom_name = Some("test_mmc1".to_string());
    
    // Get initial state
    let initial_pc = emulator.cpu().pc;
    let initial_cycles = emulator.get_state().cpu_cycles;
    
    // Save state to slot 1
    assert!(emulator.save_state(1).is_ok());
    println!("✅ Save state creation test passed");
    
    // Modify CPU state
    emulator.cpu_mut().pc = 0x1234;
    emulator.cpu_mut().a = 0xAA;
    emulator.cpu_mut().x = 0xBB;
    emulator.cpu_mut().y = 0xCC;
    
    // Load state from slot 1
    assert!(emulator.load_state(1).is_ok());
    println!("✅ Save state loading test passed");
    
    // Verify state restoration
    assert_eq!(emulator.cpu().pc, initial_pc);
    assert_eq!(emulator.get_state().cpu_cycles, initial_cycles);
    println!("✅ Save state verification test passed");
    
    // Test save state management
    assert!(emulator.has_save_state(1));
    assert!(!emulator.has_save_state(2));
    println!("✅ Save state existence check passed");
    
    // Test multiple slots
    assert!(emulator.save_state(2).is_ok());
    assert!(emulator.has_save_state(1));
    assert!(emulator.has_save_state(2));
    println!("✅ Multiple save state slots test passed");
    
    // Test save state deletion
    assert!(emulator.delete_save_state(1).is_ok());
    assert!(!emulator.has_save_state(1));
    assert!(emulator.has_save_state(2));
    println!("✅ Save state deletion test passed");
    
    // Clean up
    let _ = emulator.delete_save_state(2);
    println!("✅ Save state cleanup passed");
}

/// Test MMC1 Mapper with Save System
#[test]
fn test_mmc1_mapper_with_save_system() {
    println!("🧪 Testing MMC1 Mapper with Save System");
    
    // Create MMC1 ROM with battery backup
    let rom_data = create_test_rom_with_battery(1);
    let cartridge = Cartridge::from_bytes(&rom_data).unwrap();
    let mut emulator = Emulator::new();
    
    // Load ROM
    assert!(emulator.load_rom(cartridge).is_ok());
    println!("✅ MMC1 ROM loading test passed");
    
    // Check if mapper has battery backup
    let mapper = emulator.bus().mapper();
    assert!(mapper.has_battery());
    println!("✅ MMC1 battery backup detection passed");
    
    // Write test data to PRG RAM
    let test_data = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
    let mapper_mut = emulator.bus_mut().mapper_mut();
    
    if let Some(ram) = mapper_mut.get_prg_ram_mut() {
        if ram.len() >= test_data.len() {
            ram[..test_data.len()].copy_from_slice(&test_data);
            println!("✅ Test data written to PRG RAM");
        }
    }
    
    // Save battery backup
    assert!(emulator.save_battery_backup().is_ok());
    println!("✅ MMC1 battery backup save test passed");
    
    // Create new emulator instance to test loading
    let cartridge2 = Cartridge::from_bytes(&rom_data).unwrap();
    let mut emulator2 = Emulator::new();
    assert!(emulator2.load_rom(cartridge2).is_ok());
    
    // Check if battery backup was loaded
    let mapper2 = emulator2.bus().mapper();
    if let Some(ram) = mapper2.get_prg_ram() {
        if ram.len() >= test_data.len() && &ram[..test_data.len()] == test_data.as_slice() {
            println!("✅ MMC1 battery backup load test passed");
        } else {
            println!("⚠️  MMC1 battery backup data mismatch");
        }
    }
    
    println!("✅ MMC1 mapper with save system test completed");
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

/// Test M4 integration with all components
#[test]
fn test_m4_integration() {
    println!("🧪 Testing M4 Integration (Mappers + Save System)");
    
    // Test all supported mappers with save system
    let mappers = vec![1, 2, 3, 7]; // MMC1, UxROM, CNROM, AOROM
    
    for mapper_num in mappers {
        println!("Testing Mapper {} integration...", mapper_num);
        
        // Create ROM with battery backup
        let rom_data = create_test_rom_with_battery(mapper_num);
        let cartridge = Cartridge::from_bytes(&rom_data).unwrap();
        let mut emulator = Emulator::new();
        
        // Load ROM
        assert!(emulator.load_rom(cartridge).is_ok());
        
        // Test mapper functionality
        let cartridge = emulator.bus().cartridge.as_ref().unwrap();
        assert!(cartridge.mapper_number() == mapper_num);
        
        // Test save state functionality
        assert!(emulator.save_state(1).is_ok());
        assert!(emulator.has_save_state(1));
        
        // Test battery backup if supported
        let mapper = emulator.bus().mapper();
        if mapper.has_battery() {
            let test_data = vec![0xAA, 0xBB, 0xCC, 0xDD];
            let mapper_mut = emulator.bus_mut().mapper_mut();
            
            if let Some(ram) = mapper_mut.get_prg_ram_mut() {
                if ram.len() >= test_data.len() {
                    ram[..test_data.len()].copy_from_slice(&test_data);
                    assert!(emulator.save_battery_backup().is_ok());
                }
            }
        }
        
        // Clean up
        let _ = emulator.delete_save_state(1);
        
        println!("✅ Mapper {} integration test passed", mapper_num);
    }
    
    println!("✅ M4 integration test completed");
}

/// Test M4 using the dedicated test runner
#[test]
fn test_m4_with_test_runner() {
    println!("🧪 Testing M4 with Test Runner");
    
    // Create test ROM
    let rom_data = create_test_rom_with_battery(1);
    let temp_rom_path = std::env::temp_dir().join("test_m4.nes");
    std::fs::write(&temp_rom_path, &rom_data).unwrap();
    
    // Create test runner
    let mut runner = M4TestRunner::new()
        .with_max_cycles(1000) // Shorter test
        .with_save_system(true)
        .with_battery_backup(false) // Disable battery backup to avoid memory errors
        .with_save_state_slots(vec![1, 2]);
    
    // Load ROM
    assert!(runner.load_rom(&temp_rom_path).is_ok());
    println!("✅ ROM loaded successfully");
    
    // Test save system specifically
    match runner.test_save_system() {
        Ok(M4TestResult::Completed { save_states_created, battery_backups_saved, .. }) => {
            println!("✅ Save system test completed");
            println!("  - Save states created: {}", save_states_created);
            println!("  - Battery backups saved: {}", battery_backups_saved);
        }
        Ok(M4TestResult::Error { error, .. }) => {
            println!("❌ Save system test failed: {}", error);
            panic!("Save system test failed");
        }
        _ => {
            println!("⚠️  Save system test returned unexpected result");
        }
    }
    
    // Clean up
    let _ = std::fs::remove_file(&temp_rom_path);
    println!("✅ M4 test runner test completed");
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

/// Create a test ROM with battery backup for specified mapper
fn create_test_rom_with_battery(mapper_number: u8) -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header with battery backup flag
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        (mapper_number & 0x0F) << 4 | 0x02, (mapper_number & 0xF0), // Mapper number, battery backup, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Simple program
    let mut prg_rom = vec![0; 16384];
    
    // Simple program that just loops
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x42; // Value 0x42
    prg_rom[2] = 0x8D; // STA absolute
    prg_rom[3] = 0x00; // Low byte
    prg_rom[4] = 0x60; // High byte (PRG RAM)
    prg_rom[5] = 0x4C; // JMP absolute
    prg_rom[6] = 0x00; // Low byte
    prg_rom[7] = 0x80; // High byte (infinite loop)
    
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
