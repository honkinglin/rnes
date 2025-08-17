use rnes_core::Emulator;
use rnes_cartridge::Cartridge;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES Save System Demo: Battery Backup and Save States");
    println!("=====================================================");
    
    // Create a test ROM with battery backup
    let rom_data = create_test_rom_with_battery();
    
    // Load the ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    println!("✓ Save system initialized");
    
    // Test battery backup functionality
    test_battery_backup(&mut emulator)?;
    
    // Test save state functionality
    test_save_states(&mut emulator)?;
    
    println!("\n✓ Save system demo completed successfully!");
    
    Ok(())
}

fn test_battery_backup(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 Testing Battery Backup Functionality");
    println!("{}", "-".repeat(50));
    
    // Simulate some game progress by writing to PRG RAM
    println!("Simulating game progress...");
    
    // Write some test data to PRG RAM (simulating game save data)
    let test_data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    
    // In a real implementation, this would be done through the mapper
    // For demo purposes, we'll simulate it by directly accessing the mapper
    let mapper = emulator.bus_mut().mapper_mut();
    if mapper.has_battery() {
        if let Some(ram) = mapper.get_prg_ram_mut() {
            if ram.len() >= test_data.len() {
                ram[..test_data.len()].copy_from_slice(&test_data);
                println!("✓ Test data written to PRG RAM");
            }
        }
    }
    
    // Save battery backup
    println!("Saving battery backup...");
    emulator.save_battery_backup()?;
    println!("✓ Battery backup saved");
    
    // Create a new emulator instance to test loading
    println!("Creating new emulator instance...");
    let cartridge = Cartridge::from_bytes(&create_test_rom_with_battery())?;
    let mut new_emulator = Emulator::new();
    new_emulator.load_rom(cartridge)?;
    
    // Check if battery backup was loaded
    let mapper = new_emulator.bus_mut().mapper();
    if mapper.has_battery() {
        if let Some(ram) = mapper.get_prg_ram() {
            if ram.len() >= test_data.len() && &ram[..test_data.len()] == test_data.as_slice() {
                println!("✓ Battery backup loaded successfully");
                println!("  Test data verified: {:02X?}", &ram[..test_data.len()]);
            } else {
                println!("⚠️  Battery backup data mismatch");
            }
        }
    }
    
    Ok(())
}

fn test_save_states(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 Testing Save State Functionality");
    println!("{}", "-".repeat(50));
    
    // Get initial state
    let initial_pc = emulator.cpu().pc;
    let initial_cycles = emulator.get_state().cpu_cycles;
    println!("Initial state - PC: 0x{:04X}, Cycles: {}", initial_pc, initial_cycles);
    
    // Save state to slot 1
    println!("Saving state to slot 1...");
    emulator.save_state(1)?;
    println!("✓ Save state saved to slot 1");
    
    // Modify CPU state to test restoration
    emulator.cpu_mut().pc = 0x1234;
    emulator.cpu_mut().a = 0xAA;
    emulator.cpu_mut().x = 0xBB;
    emulator.cpu_mut().y = 0xCC;
    
    let modified_pc = emulator.cpu().pc;
    println!("Modified state - PC: 0x{:04X}", modified_pc);
    
    // Load state from slot 1
    println!("Loading state from slot 1...");
    emulator.load_state(1)?;
    println!("✓ Save state loaded from slot 1");
    
    // Verify the state was restored
    let restored_pc = emulator.cpu().pc;
    let restored_cycles = emulator.get_state().cpu_cycles;
    println!("Restored state - PC: 0x{:04X}, Cycles: {}", restored_pc, restored_cycles);
    
    if restored_pc == initial_pc && restored_cycles == initial_cycles {
        println!("✓ Save state verification successful");
    } else {
        println!("⚠️  Save state verification failed");
    }
    
    // Test save state management
    println!("\nTesting save state management...");
    
    // Check if save state exists
    if emulator.has_save_state(1) {
        println!("✓ Save state exists in slot 1");
    } else {
        println!("⚠️  Save state not found in slot 1");
    }
    
    // Check if save state exists in slot 2 (should not exist)
    if !emulator.has_save_state(2) {
        println!("✓ Slot 2 correctly shows no save state");
    } else {
        println!("⚠️  Unexpected save state found in slot 2");
    }
    
    // Save to slot 2
    println!("Saving state to slot 2...");
    emulator.save_state(2)?;
    println!("✓ Save state saved to slot 2");
    
    // Verify both slots now have save states
    if emulator.has_save_state(1) && emulator.has_save_state(2) {
        println!("✓ Both slots 1 and 2 have save states");
    }
    
    // Delete save state from slot 1
    println!("Deleting save state from slot 1...");
    emulator.delete_save_state(1)?;
    println!("✓ Save state deleted from slot 1");
    
    // Verify slot 1 is empty but slot 2 still exists
    if !emulator.has_save_state(1) && emulator.has_save_state(2) {
        println!("✓ Slot 1 empty, slot 2 still has save state");
    } else {
        println!("⚠️  Save state deletion verification failed");
    }
    
    emulator.stop();
    Ok(())
}

fn create_test_rom_with_battery() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header with battery backup flag
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x12, 0x00,             // Mapper 1 (MMC1), battery backup, horizontal mirroring
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
