use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Starting simple APU test...");
    
    // Create a minimal test cartridge
    let mut cartridge_data = vec![
        // iNES header
        0x4E, 0x45, 0x53, 0x1A, // "NES" + EOF
        0x01, 0x00,             // 16KB PRG ROM, 0KB CHR ROM
        0x00, 0x00,             // Mapper 0, no special features
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ];
    
    // Add 16KB of PRG ROM (mostly zeros, with a simple program)
    let mut prg_rom = vec![0; 16384];
    
    // Simple program that enables APU channels and plays some tones
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x0F; // Value 0x0F
    prg_rom[2] = 0x8D; // STA absolute
    prg_rom[3] = 0x15; // Low byte of 0x4015
    prg_rom[4] = 0x40; // High byte of 0x4015
    
    // Enable pulse 1 channel
    prg_rom[5] = 0xA9; // LDA immediate
    prg_rom[6] = 0x3F; // Volume 15, constant volume
    prg_rom[7] = 0x8D; // STA absolute
    prg_rom[8] = 0x00; // Low byte of 0x4000
    prg_rom[9] = 0x40; // High byte of 0x4000
    
    // Set pulse 1 frequency (low)
    prg_rom[10] = 0xA9; // LDA immediate
    prg_rom[11] = 0xFF; // Frequency value
    prg_rom[12] = 0x8D; // STA absolute
    prg_rom[13] = 0x02; // Low byte of 0x4002
    prg_rom[14] = 0x40; // High byte of 0x4002
    
    // Set pulse 1 frequency (high) and length
    prg_rom[15] = 0xA9; // LDA immediate
    prg_rom[16] = 0x07; // High frequency + length
    prg_rom[17] = 0x8D; // STA absolute
    prg_rom[18] = 0x03; // Low byte of 0x4003
    prg_rom[19] = 0x40; // High byte of 0x4003
    
    // Infinite loop
    prg_rom[20] = 0x4C; // JMP absolute
    prg_rom[21] = 0x14; // Low byte of 0x8014
    prg_rom[22] = 0x80; // High byte of 0x8014
    
    cartridge_data.extend(prg_rom);
    
    // Create cartridge from data
    let cartridge = Cartridge::from_bytes(&cartridge_data)?;
    println!("Test cartridge created");
    
    // Create emulator
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("Running APU test for 100 frames...");
    
    // Run emulator for a few frames to generate audio
    emulator.start();
    
    let mut frame_count = 0;
    let target_frames = 100;
    let mut total_samples = 0;
    
    while frame_count < target_frames {
        // Run one frame (29780 CPU cycles)
        emulator.run_cycles(29780)?;
        
        // Get audio samples
        let samples = emulator.get_audio_samples();
        if !samples.is_empty() {
            total_samples += samples.len();
            println!("Frame {}: Generated {} audio samples", frame_count, samples.len());
            
            // Print first few sample values for debugging
            if frame_count < 3 {
                println!("  Sample values: {:?}", &samples[..samples.len().min(5)]);
            }
        }
        
        frame_count += 1;
    }
    
    emulator.stop();
    println!("APU test completed!");
    println!("Total audio samples generated: {}", total_samples);
    
    // Print APU status
    let apu = emulator.apu();
    let status = apu.read_register(0x4015)?;
    println!("APU Status: 0x{:02X}", status);
    
    // Check if pulse 1 is active
    if status & 0x01 != 0 {
        println!("✓ Pulse 1 channel is active");
    } else {
        println!("✗ Pulse 1 channel is not active");
    }
    
    Ok(())
}
