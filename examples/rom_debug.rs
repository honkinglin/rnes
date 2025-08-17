use rnes_core::Emulator;
use rnes_cartridge::Cartridge;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ROM Debug Test");
    println!("==============");
    
    // Create a simple test ROM
    let rom_data = create_test_rom();
    println!("Created test ROM with {} bytes", rom_data.len());
    
    // Load ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    println!("✓ ROM loaded successfully");
    println!("  PRG ROM size: {} bytes", cartridge.prg_rom.len());
    println!("  CHR ROM size: {} bytes", cartridge.chr_rom.len());
    
    // Check first few bytes of PRG ROM
    println!("  First 16 bytes of PRG ROM:");
    for i in 0..16 {
        if i < cartridge.prg_rom.len() {
            print!("  {:02X}", cartridge.prg_rom[i]);
        }
    }
    println!();
    
    // Check reset vector
    let reset_low = cartridge.read_prg_rom(0x3FFC)?;
    let reset_high = cartridge.read_prg_rom(0x3FFD)?;
    let reset_vector = (reset_high as u16) << 8 | (reset_low as u16);
    println!("  Reset vector: 0x{:04X} (low: 0x{:02X}, high: 0x{:02X})", 
             reset_vector, reset_low, reset_high);
    
    // Create emulator and load ROM
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded into emulator");
    println!("  CPU PC: 0x{:04X}", emulator.cpu().pc);
    
    // Check what's at PC address
    let instruction_at_pc = emulator.bus.read_byte(emulator.cpu().pc)?;
    println!("  Instruction at PC (0x{:04X}): 0x{:02X}", emulator.cpu().pc, instruction_at_pc);
    
    // Check what's at address 0x8000
    let instruction_at_8000 = emulator.bus.read_byte(0x8000)?;
    println!("  Instruction at 0x8000: 0x{:02X}", instruction_at_8000);
    
    // Check what's at address 0x8001
    let instruction_at_8001 = emulator.bus.read_byte(0x8001)?;
    println!("  Instruction at 0x8001: 0x{:02X}", instruction_at_8001);
    
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
    
    // PRG ROM (16KB) - Simple program
    let mut prg_rom = vec![0; 16384];
    
    // Program: LDA #$42, TAX, LDA #$84, TAY, JMP $8000
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x42; // Value 0x42
    prg_rom[2] = 0xAA; // TAX
    prg_rom[3] = 0xA9; // LDA immediate
    prg_rom[4] = 0x84; // Value 0x84
    prg_rom[5] = 0xA8; // TAY
    prg_rom[6] = 0x4C; // JMP absolute
    prg_rom[7] = 0x00; // Low byte
    prg_rom[8] = 0x80; // High byte (infinite loop)
    
    // Reset vector
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Empty
    rom.extend(vec![0; 8192]);
    
    rom
}
