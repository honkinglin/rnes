use anyhow::Result;
use clap::Parser;
use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "rnes")]
#[command(about = "NES emulator written in Rust")]
struct Args {
    /// ROM file to load
    #[arg(value_name = "ROM_FILE")]
    rom_file: Option<String>,
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    let args = Args::parse();
    
    info!("RNES - NES Emulator Starting");
    
    let mut emulator = Emulator::new();
    
    let has_rom = args.rom_file.is_some();
    
    if let Some(rom_path) = args.rom_file {
        info!("Loading ROM: {}", rom_path);
        match Cartridge::from_file(&rom_path) {
            Ok(cartridge) => {
                if let Err(e) = emulator.load_rom(cartridge) {
                    error!("Failed to load ROM: {}", e);
                    return Err(e.into());
                }
                info!("ROM loaded successfully");
            }
            Err(e) => {
                error!("Failed to read ROM file: {}", e);
                return Err(e.into());
            }
        }
    } else {
        info!("No ROM file specified, starting debug mode");
    }
    
    // Simple test loop
    emulator.start();
    
    // If no ROM, run demo mode
    if !has_rom {
        info!("Running demo mode");
        info!("Initializing memory...");
        
        // Initialize some test data in memory
        emulator.bus.write_byte(0x0000, 0xA9).unwrap(); // LDA immediate
        emulator.bus.write_byte(0x0001, 0x42).unwrap(); // Value 0x42
        emulator.bus.write_byte(0x0002, 0x85).unwrap(); // STA zero page
        emulator.bus.write_byte(0x0003, 0x00).unwrap(); // Address 0x00
        emulator.bus.write_byte(0x0004, 0x4C).unwrap(); // JMP absolute
        emulator.bus.write_byte(0x0005, 0x00).unwrap(); // Low byte
        emulator.bus.write_byte(0x0006, 0x00).unwrap(); // High byte
        
        // Set reset vector to point to our program
        emulator.bus.write_byte(0xFFFC, 0x00).unwrap(); // Low byte
        emulator.bus.write_byte(0xFFFD, 0x00).unwrap(); // High byte
        
        info!("Memory initialization complete");
        
        // Reset CPU to read reset vector
        emulator.reset().unwrap();
        
        info!("CPU reset complete, PC = 0x{:04X}", emulator.cpu.pc);
        info!("Memory[0x0000] = 0x{:02X}", emulator.bus.read_byte(0x0000).unwrap());
        info!("Memory[0x0001] = 0x{:02X}", emulator.bus.read_byte(0x0001).unwrap());
        
        for i in 0..10 {
            if let Err(e) = emulator.step() {
                error!("Emulator error: {}", e);
                break;
            }
            
            info!("Step {}: CPU status: {}", i, emulator.cpu_status());
            info!("Memory[0x00] = 0x{:02X}", emulator.bus.read_byte(0x0000).unwrap());
        }
    } else {
        // Run ROM
        for i in 0..1000 {
            if let Err(e) = emulator.step() {
                error!("Emulator error: {}", e);
                break;
            }
            
            if i % 100 == 0 {
                info!("CPU status: {}", emulator.cpu_status());
            }
        }
    }
    
    info!("Emulator run complete");
    Ok(())
}
