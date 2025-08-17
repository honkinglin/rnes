use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 RNES Memory Access Debug");
    println!("===========================");
    
    // Create a simple test ROM
    let rom_data = create_debug_rom();
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    
    // Run the emulator step by step with memory access tracing
    emulator.start();
    
    println!("\nRunning emulator with memory access tracing...");
    for step in 0..50 {
        match emulator.step() {
            Ok(_) => {
                let state = emulator.get_state();
                if step % 10 == 0 {
                    println!("Step {}: CPU cycles: {}, PPU scanline: {}, PPU dot: {}", 
                            step, state.cpu_cycles, state.ppu_scanline, state.ppu_dot);
                }
            }
            Err(e) => {
                println!("Step {}: ERROR - {}", step, e);
                break;
            }
        }
    }
    
    emulator.stop();
    Ok(())
}

fn create_debug_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Very simple program that reads PPUSTATUS
    let mut prg_rom = vec![0; 16384];
    
    // Program: Read PPUSTATUS to check VBlank
    prg_rom[0] = 0xAD; // LDA absolute
    prg_rom[1] = 0x02; // Low byte
    prg_rom[2] = 0x20; // High byte (PPUSTATUS)
    prg_rom[3] = 0x4C; // JMP absolute
    prg_rom[4] = 0x00; // Low byte
    prg_rom[5] = 0x80; // High byte (infinite loop)
    
    // Reset vector
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Empty
    rom.extend(vec![0; 8192]);
    
    rom
}
