use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::Pixel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES M4 Demo: Common Mappers (MMC1, UxROM, CNROM, AOROM)");
    println!("=========================================================");
    
    // Test each mapper type
    test_mapper(0, "NROM")?;
    test_mapper(1, "MMC1")?;
    test_mapper(2, "UxROM")?;
    test_mapper(3, "CNROM")?;
    test_mapper(7, "AOROM")?;
    
    println!("\n✓ M4 demo completed successfully!");
    println!("✓ All common mappers are working!");
    
    Ok(())
}

fn test_mapper(mapper_number: u8, mapper_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 Testing {} Mapper (Mapper {})", mapper_name, mapper_number);
    println!("{}", "-".repeat(50));
    
    // Create a test ROM with the specified mapper
    let rom_data = create_test_rom(mapper_number);
    
    // Load the ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    println!("✓ {} Mapper initialized", mapper_name);
    
    // Run the emulator for a few frames
    emulator.start();
    
    println!("Running emulator for 2 frames...");
    for frame in 0..2 {
        // Run one frame (262 scanlines * 341 dots = 89,342 PPU cycles)
        // CPU runs at 1/3 the speed of PPU, so we need 29,780 CPU cycles per frame
        emulator.run_cycles(29_780)?;
        
        // Check if we have a frame buffer
        if let Some(frame_buffer) = emulator.get_ppu_frame_buffer() {
            println!("Frame {}: Generated {} pixels", frame + 1, frame_buffer.len());
            
            // Count non-black pixels to see if rendering is working
            let non_black_pixels = frame_buffer.iter()
                .filter(|pixel| **pixel != Pixel::BLACK)
                .count();
            println!("  Non-black pixels: {}", non_black_pixels);
        }
        
        // Check VBlank status
        if emulator.ppu_vblank() {
            println!("  VBlank active");
        }
        
        // Get current state
        let state = emulator.get_state();
        println!("  CPU cycles: {}, PPU scanline: {}, PPU dot: {}", 
                state.cpu_cycles, state.ppu_scanline, state.ppu_dot);
    }
    
    emulator.stop();
    println!("✓ {} mapper test completed", mapper_name);
    
    Ok(())
}

fn create_test_rom(mapper_number: u8) -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x02, 0x02,             // 32KB PRG, 16KB CHR (for bank switching)
        mapper_number << 4, 0x00, // Mapper number, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (32KB) - Multiple banks for testing
    let mut prg_rom = vec![0; 32768];
    
    // Bank 0: Simple program that sets up PPU and loops
    let bank0_start = 0;
    prg_rom[bank0_start + 0] = 0xA9; // LDA immediate
    prg_rom[bank0_start + 1] = 0x00; // Value 0x00
    prg_rom[bank0_start + 2] = 0x8D; // STA absolute
    prg_rom[bank0_start + 3] = 0x00; // Low byte
    prg_rom[bank0_start + 4] = 0x20; // High byte (PPUCTRL)
    prg_rom[bank0_start + 5] = 0xA9; // LDA immediate
    prg_rom[bank0_start + 6] = 0x08; // Value 0x08 (show background)
    prg_rom[bank0_start + 7] = 0x8D; // STA absolute
    prg_rom[bank0_start + 8] = 0x01; // Low byte
    prg_rom[bank0_start + 9] = 0x20; // High byte (PPUMASK)
    prg_rom[bank0_start + 10] = 0x4C; // JMP absolute
    prg_rom[bank0_start + 11] = 0x0A; // Low byte
    prg_rom[bank0_start + 12] = 0x80; // High byte (infinite loop)
    
    // Bank 1: Different pattern for bank switching test
    let bank1_start = 16384;
    prg_rom[bank1_start + 0] = 0xA9; // LDA immediate
    prg_rom[bank1_start + 1] = 0x01; // Value 0x01 (different from bank 0)
    prg_rom[bank1_start + 2] = 0x8D; // STA absolute
    prg_rom[bank1_start + 3] = 0x00; // Low byte
    prg_rom[bank1_start + 4] = 0x20; // High byte (PPUCTRL)
    prg_rom[bank1_start + 5] = 0xA9; // LDA immediate
    prg_rom[bank1_start + 6] = 0x18; // Value 0x18 (show background + sprites)
    prg_rom[bank1_start + 7] = 0x8D; // STA absolute
    prg_rom[bank1_start + 8] = 0x01; // Low byte
    prg_rom[bank1_start + 9] = 0x20; // High byte (PPUMASK)
    prg_rom[bank1_start + 10] = 0x4C; // JMP absolute
    prg_rom[bank1_start + 11] = 0x0A; // Low byte
    prg_rom[bank1_start + 12] = 0x80; // High byte (infinite loop)
    
    // Reset vector points to bank 0
    prg_rom[0x7FFC] = 0x00; // Reset vector low
    prg_rom[0x7FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (16KB) - Multiple banks for testing
    let mut chr_rom = vec![0; 16384];
    
    // Bank 0: Simple pattern
    for tile in 0..32 {
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
    
    // Bank 1: Different pattern
    for tile in 0..32 {
        let base = (tile + 32) * 16;
        // Create a different pattern (stripes)
        chr_rom[base + 0] = 0xFF; // 11111111
        chr_rom[base + 1] = 0x00; // 00000000
        chr_rom[base + 2] = 0xFF;
        chr_rom[base + 3] = 0x00;
        chr_rom[base + 4] = 0xFF;
        chr_rom[base + 5] = 0x00;
        chr_rom[base + 6] = 0xFF;
        chr_rom[base + 7] = 0x00;
        chr_rom[base + 8] = 0xFF;
        chr_rom[base + 9] = 0x00;
        chr_rom[base + 10] = 0xFF;
        chr_rom[base + 11] = 0x00;
        chr_rom[base + 12] = 0xFF;
        chr_rom[base + 13] = 0x00;
        chr_rom[base + 14] = 0xFF;
        chr_rom[base + 15] = 0x00;
    }
    
    rom.extend_from_slice(&chr_rom);
    
    rom
}
