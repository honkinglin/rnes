use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::Pixel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES M1 Simple Background Test: Direct PPU Setup");
    println!("================================================");
    
    // Create a simple test ROM
    let rom_data = create_simple_background_rom();
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    
    // Run the emulator for a few steps
    emulator.start();
    
    println!("\nInitial PPU state:");
    print_ppu_state(&emulator);
    
    // Run a few CPU cycles to let the program execute
    for i in 0..50 {
        emulator.step()?;
        if i % 10 == 0 {
            println!("\nAfter step {}:", i + 1);
            print_ppu_state(&emulator);
        }
    }
    
    emulator.stop();
    Ok(())
}

fn print_ppu_state(emulator: &Emulator) {
    if let Some(registers) = emulator.debug_ppu_registers() {
        println!("  PPUCTRL: 0x{:02X}, PPUMASK: 0x{:02X}, PPUSTATUS: 0x{:02X}", 
                registers.ppuctrl, registers.ppumask, registers.ppustatus);
    }
    
    if let Some(state) = emulator.debug_ppu_state() {
        println!("  PPU v: 0x{:04X}, t: 0x{:04X}, x: {}", 
                state.v, state.t, state.x);
    }
    
    println!("  Background enabled: {}", emulator.debug_ppu_background_enabled());
    println!("  VBlank: {}", emulator.ppu_vblank());
    
    if let Some(frame_buffer) = emulator.get_ppu_frame_buffer() {
        let non_black_pixels = frame_buffer.iter()
            .filter(|pixel| **pixel != Pixel::BLACK)
            .count();
        println!("  Non-black pixels: {}", non_black_pixels);
        
        if non_black_pixels > 0 {
            println!("  ✓ SUCCESS: Found non-black pixels!");
        }
    }
}

fn create_simple_background_rom() -> Vec<u8> {
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
    
    let mut offset = 0;
    
    // Set PPUCTRL to enable background rendering
    prg_rom[offset] = 0xA9; // LDA immediate
    offset += 1;
    prg_rom[offset] = 0x08; // Value 0x08 (enable background)
    offset += 1;
    prg_rom[offset] = 0x8D; // STA absolute
    offset += 1;
    prg_rom[offset] = 0x00; // Low byte
    offset += 1;
    prg_rom[offset] = 0x20; // High byte (PPUCTRL)
    offset += 1;
    
    // Set PPUMASK to show background
    prg_rom[offset] = 0xA9; // LDA immediate
    offset += 1;
    prg_rom[offset] = 0x08; // Value 0x08 (show background)
    offset += 1;
    prg_rom[offset] = 0x8D; // STA absolute
    offset += 1;
    prg_rom[offset] = 0x01; // Low byte
    offset += 1;
    prg_rom[offset] = 0x20; // High byte (PPUMASK)
    offset += 1;
    
    // Set PPU address to nametable 0
    prg_rom[offset] = 0xA9; // LDA immediate
    offset += 1;
    prg_rom[offset] = 0x20; // High byte of nametable address
    offset += 1;
    prg_rom[offset] = 0x8D; // STA absolute
    offset += 1;
    prg_rom[offset] = 0x06; // Low byte
    offset += 1;
    prg_rom[offset] = 0x20; // High byte (PPUADDR)
    offset += 1;
    
    prg_rom[offset] = 0xA9; // LDA immediate
    offset += 1;
    prg_rom[offset] = 0x00; // Low byte of nametable address
    offset += 1;
    prg_rom[offset] = 0x8D; // STA absolute
    offset += 1;
    prg_rom[offset] = 0x06; // Low byte
    offset += 1;
    prg_rom[offset] = 0x20; // High byte (PPUADDR)
    offset += 1;
    
    // Write some tile data to nametable (write tile 0x01 to first few positions)
    for i in 0..16 {
        prg_rom[offset] = 0xA9; // LDA immediate
        offset += 1;
        prg_rom[offset] = 0x01; // Tile ID 0x01
        offset += 1;
        prg_rom[offset] = 0x8D; // STA absolute
        offset += 1;
        prg_rom[offset] = 0x07; // Low byte
        offset += 1;
        prg_rom[offset] = 0x20; // High byte (PPUDATA)
        offset += 1;
    }
    
    // Infinite loop
    prg_rom[offset] = 0x4C; // JMP absolute
    offset += 1;
    prg_rom[offset] = 0x00; // Low byte
    offset += 1;
    prg_rom[offset] = 0x80; // High byte (infinite loop)
    offset += 1;
    
    // Reset vector points to our program
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Pattern data
    let mut chr_rom = vec![0; 8192];
    
    // Create pattern for tile 0x01 (a simple pattern)
    let base = 0x01 * 16;
    chr_rom[base + 0] = 0xFF; // 11111111
    chr_rom[base + 1] = 0x81; // 10000001
    chr_rom[base + 2] = 0x81; // 10000001
    chr_rom[base + 3] = 0x81; // 10000001
    chr_rom[base + 4] = 0x81; // 10000001
    chr_rom[base + 5] = 0x81; // 10000001
    chr_rom[base + 6] = 0x81; // 10000001
    chr_rom[base + 7] = 0xFF; // 11111111
    
    // Pattern 1: same as pattern 0 for now
    chr_rom[base + 8] = 0xFF; // 11111111
    chr_rom[base + 9] = 0x81; // 10000001
    chr_rom[base + 10] = 0x81; // 10000001
    chr_rom[base + 11] = 0x81; // 10000001
    chr_rom[base + 12] = 0x81; // 10000001
    chr_rom[base + 13] = 0x81; // 10000001
    chr_rom[base + 14] = 0x81; // 10000001
    chr_rom[base + 15] = 0xFF; // 11111111
    
    rom.extend_from_slice(&chr_rom);
    
    rom
}
