use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        std::process::exit(1);
    }
    
    let rom_path = &args[1];
    println!("Loading ROM: {}", rom_path);
    
    // Load cartridge
    let cartridge = Cartridge::from_file(rom_path)?;
    println!("Cartridge loaded: {:?}", cartridge.header);
    
    // Create emulator
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("Starting APU demo...");
    println!("Running for 1000 frames to generate audio...");
    
    // Run emulator for a few frames to generate audio
    emulator.start();
    
    let mut frame_count = 0;
    let target_frames = 1000;
    
    while frame_count < target_frames {
        // Run one frame (29780 CPU cycles)
        emulator.run_cycles(29780)?;
        
        // Get audio samples
        let samples = emulator.get_audio_samples();
        if !samples.is_empty() {
            println!("Frame {}: Generated {} audio samples", frame_count, samples.len());
            
            // Print first few sample values for debugging
            if frame_count < 5 {
                println!("  Sample values: {:?}", &samples[..samples.len().min(10)]);
            }
        }
        
        frame_count += 1;
    }
    
    emulator.stop();
    println!("APU demo completed!");
    
    // Print APU status
    let apu = emulator.apu();
    println!("APU Status: {:?}", apu.read_register(0x4015)?);
    
    Ok(())
}
