use rnes_core::Emulator;
use rnes_cartridge::Cartridge;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 RNES CPU Execution Test");
    println!("==========================");
    
    // Create a simple test ROM
    let rom_data = create_execution_test_rom();
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
                println!("Step {}: CPU cycles: {}, PC: 0x{:04X}", 
                        step, state.cpu_cycles, emulator.cpu.pc);
                
                // Check PPU registers
                if let Some(registers) = emulator.debug_ppu_registers() {
                    println!("  PPUCTRL: 0x{:02X}, PPUMASK: 0x{:02X}", 
                            registers.ppuctrl, registers.ppumask);
                }
                
                // Check CPU status
                println!("  CPU: {}", emulator.cpu_status());
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

fn create_execution_test_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Simple program to set PPU registers
    let mut prg_rom = vec![0; 16384];
    
    // Program at 0x8000:
    // LDA #$08     ; Load 0x08
    // STA $2000    ; Store to PPUCTRL
    // LDA #$08     ; Load 0x08  
    // STA $2001    ; Store to PPUMASK
    // JMP $8000    ; Infinite loop
    
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x08; // Value 0x08
    prg_rom[2] = 0x8D; // STA absolute
    prg_rom[3] = 0x00; // Low byte
    prg_rom[4] = 0x20; // High byte (PPUCTRL)
    prg_rom[5] = 0xA9; // LDA immediate
    prg_rom[6] = 0x08; // Value 0x08
    prg_rom[7] = 0x8D; // STA absolute
    prg_rom[8] = 0x01; // Low byte
    prg_rom[9] = 0x20; // High byte (PPUMASK)
    prg_rom[10] = 0x4C; // JMP absolute
    prg_rom[11] = 0x00; // Low byte
    prg_rom[12] = 0x80; // High byte (infinite loop)
    
    // Reset vector points to our program
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Empty
    rom.extend(vec![0; 8192]);
    
    rom
}
