use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::Pixel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES M1 Demo: PPU Background Rendering + NROM Mapper");
    println!("==================================================");
    
    // Create a simple test ROM with some pattern data
    let rom_data = create_test_rom();
    
    // Load the ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    println!("✓ NROM Mapper initialized");
    println!("✓ PPU initialized with background rendering");
    
    // Run the emulator for a few frames
    emulator.start();
    
    println!("\nRunning emulator for 3 frames...");
    for frame in 0..3 {
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
    println!("\n✓ M1 demo completed successfully!");
    
    Ok(())
}

fn create_test_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Simple program that sets up PPU
    let mut prg_rom = vec![0; 16384];
    
    // Simple program to initialize PPU and set up some background
    // This is a very basic program that just sets some PPU registers
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x00; // Value 0x00
    prg_rom[2] = 0x8D; // STA absolute
    prg_rom[3] = 0x00; // Low byte
    prg_rom[4] = 0x20; // High byte (PPUCTRL)
    prg_rom[5] = 0xA9; // LDA immediate
    prg_rom[6] = 0x08; // Value 0x08 (show background)
    prg_rom[7] = 0x8D; // STA absolute
    prg_rom[8] = 0x01; // Low byte
    prg_rom[9] = 0x20; // High byte (PPUMASK)
    prg_rom[10] = 0x4C; // JMP absolute
    prg_rom[11] = 0x0A; // Low byte
    prg_rom[12] = 0x80; // High byte (infinite loop)
    
    // Reset vector points to our program (at end of PRG ROM)
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Simple pattern data
    let mut chr_rom = vec![0; 8192];
    
    // Create a simple pattern (8x8 pixel tile)
    // This creates a simple checkerboard pattern
    for tile in 0..16 {
        let base = tile * 16;
        
        // Pattern 0: alternating pixels
        chr_rom[base + 0] = 0xAA; // 10101010
        chr_rom[base + 1] = 0x55; // 01010101
        chr_rom[base + 2] = 0xAA; // 10101010
        chr_rom[base + 3] = 0x55; // 01010101
        chr_rom[base + 4] = 0xAA; // 10101010
        chr_rom[base + 5] = 0x55; // 01010101
        chr_rom[base + 6] = 0xAA; // 10101010
        chr_rom[base + 7] = 0x55; // 01010101
        
        // Pattern 1: same as pattern 0 for now
        chr_rom[base + 8] = 0xAA; // 10101010
        chr_rom[base + 9] = 0x55; // 01010101
        chr_rom[base + 10] = 0xAA; // 10101010
        chr_rom[base + 11] = 0x55; // 01010101
        chr_rom[base + 12] = 0xAA; // 10101010
        chr_rom[base + 13] = 0x55; // 01010101
        chr_rom[base + 14] = 0xAA; // 10101010
        chr_rom[base + 15] = 0x55; // 01010101
    }
    
    rom.extend_from_slice(&chr_rom);
    
    rom
}
