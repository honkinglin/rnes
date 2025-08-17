use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::Pixel;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <rom_path>", args[0]);
        return Ok(());
    }
    
    let rom_path = &args[1];
    println!("RNES M1 Advanced ROM Test: Detailed Analysis");
    println!("============================================");
    println!("Testing ROM: {}", rom_path);
    
    // Load ROM
    let rom_data = std::fs::read(rom_path)?;
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    
    println!("✓ ROM loaded successfully");
    println!("  Mapper: {}", cartridge.mapper_number());
    println!("  Mirroring: {:?}", cartridge.mirroring());
    println!("  PRG ROM: {} bytes", cartridge.prg_rom.len());
    println!("  CHR ROM: {} bytes", cartridge.chr_rom.len());
    
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("\nCPU after emulator reset: {}", emulator.cpu_status());
    
    // Run the emulator step by step
    emulator.start();
    
    println!("\nRunning emulator step by step...");
    let mut last_ppuctrl = 0;
    let mut last_ppumask = 0;
    let mut last_ppustatus = 0;
    let mut last_pc = emulator.cpu.pc;
    
    for step in 0..1000 {
        emulator.step()?;
        
        // Check CPU state
        let cpu_state = emulator.cpu_status();
        if !cpu_state.contains("PC:") {
            continue; // Skip if CPU state is not available
        }
        
        // Extract PC from CPU state
        if let Some(pc_start) = cpu_state.find("PC:") {
            if let Some(pc_end) = cpu_state[pc_start..].find(" ") {
                if let Ok(pc) = cpu_state[pc_start + 3..pc_start + pc_end].parse::<u16>() {
                    if pc != last_pc {
                        println!("\nStep {}: CPU PC changed 0x{:04X} -> 0x{:04X}", step, last_pc, pc);
                        println!("  CPU: {}", cpu_state);
                        last_pc = pc;
                    }
                }
            }
        }
        
        if let Some(registers) = emulator.debug_ppu_registers() {
            if registers.ppuctrl != last_ppuctrl || 
               registers.ppumask != last_ppumask || 
               registers.ppustatus != last_ppustatus {
                
                println!("\nStep {}: PPU registers changed", step);
                println!("  PPUCTRL: 0x{:02X} -> 0x{:02X}", last_ppuctrl, registers.ppuctrl);
                println!("  PPUMASK: 0x{:02X} -> 0x{:02X}", last_ppumask, registers.ppumask);
                println!("  PPUSTATUS: 0x{:02X} -> 0x{:02X}", last_ppustatus, registers.ppustatus);
                println!("  CPU: {}", emulator.cpu_status());
                
                last_ppuctrl = registers.ppuctrl;
                last_ppumask = registers.ppumask;
                last_ppustatus = registers.ppustatus;
                
                // Check frame buffer
                if let Some(frame_buffer) = emulator.get_ppu_frame_buffer() {
                    let non_black_pixels = frame_buffer.iter()
                        .filter(|pixel| **pixel != Pixel::BLACK)
                        .count();
                    println!("  Non-black pixels: {}", non_black_pixels);
                    
                    if non_black_pixels > 0 {
                        println!("  ✓ SUCCESS: Found non-black pixels!");
                        break;
                    }
                }
            }
        }
        
        // Stop if we've run too many steps
        if step > 2000 {
            println!("\nStopping after 2000 steps - no PPU activity detected");
            println!("Last CPU state: {}", emulator.cpu_status());
            break;
        }
    }
    
    emulator.stop();
    Ok(())
}
