use rnes_core::Emulator;
use rnes_cartridge::Cartridge;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES M1 Simple Test: Step by Step Debug");
    println!("=======================================");
    
    // Create a very simple test ROM
    let rom_data = create_simple_rom();
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    
    // Run the emulator step by step
    emulator.start();
    
    println!("\nRunning emulator step by step...");
    for step in 0..20 {
        match emulator.step() {
            Ok(_cycles) => {
                let state = emulator.get_state();
                println!("Step {}: CPU cycles: {}, PPU scanline: {}, PPU dot: {}", 
                        step + 1, state.cpu_cycles, state.ppu_scanline, state.ppu_dot);
                
                // Check PPU registers
                if let Some(registers) = emulator.debug_ppu_registers() {
                    println!("  PPUCTRL: 0x{:02X}, PPUMASK: 0x{:02X}", 
                            registers.ppuctrl, registers.ppumask);
                }
            }
            Err(e) => {
                println!("Step {}: ERROR - {}", step + 1, e);
                break;
            }
        }
    }
    
    emulator.stop();
    Ok(())
}

fn create_simple_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Very simple program
    let mut prg_rom = vec![0; 16384];
    
    // Simple program: just set PPUCTRL
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x08; // Value 0x08
    prg_rom[2] = 0x8D; // STA absolute
    prg_rom[3] = 0x00; // Low byte
    prg_rom[4] = 0x20; // High byte (PPUCTRL)
    prg_rom[5] = 0x4C; // JMP absolute
    prg_rom[6] = 0x05; // Low byte
    prg_rom[7] = 0x80; // High byte (infinite loop)
    
    // Reset vector
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Empty
    rom.extend(vec![0; 8192]);
    
    rom
}
