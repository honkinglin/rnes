use rnes_core::Emulator;
use rnes_cartridge::Cartridge;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CPU Step Debug Test");
    println!("==================");
    
    // Create a simple test ROM
    let rom_data = create_test_rom();
    println!("Created test ROM with {} bytes", rom_data.len());
    
    // Load ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    println!("✓ ROM loaded successfully");
    println!("  PRG ROM size: {} bytes", cartridge.prg_rom.len());
    println!("  CHR ROM size: {} bytes", cartridge.chr_rom.len());
    
    // Create emulator and load ROM
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded into emulator");
    println!("  CPU PC: 0x{:04X}", emulator.cpu().pc);
    println!("  CPU A: 0x{:02X}", emulator.cpu().a);
    println!("  CPU X: 0x{:02X}", emulator.cpu().x);
    println!("  CPU Y: 0x{:02X}", emulator.cpu().y);
    
    // Start emulator
    emulator.start();
    println!("✓ Emulator started");
    println!("  Emulator running: {}", emulator.is_running());
    
    // Step through a few instructions
    for i in 0..10 {
        let cycles = emulator.step()?;
        println!("Step {}: cycles={}, PC=0x{:04X}, A=0x{:02X}, X=0x{:02X}, Y=0x{:02X}", 
                i, cycles, emulator.cpu().pc, emulator.cpu().a, emulator.cpu().x, emulator.cpu().y);
        
        if cycles == 0 {
            println!("  Warning: Step returned 0 cycles!");
            break;
        }
    }
    
    emulator.stop();
    println!("✓ Emulator stopped");
    
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
