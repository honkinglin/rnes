use rnes_core::Emulator;
use rnes_cartridge::Cartridge;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Starting APU register test...");
    
    // Create a minimal test cartridge
    let mut cartridge_data = vec![
        // iNES header
        0x4E, 0x45, 0x53, 0x1A, // "NES" + EOF
        0x01, 0x00,             // 16KB PRG ROM, 0KB CHR ROM
        0x00, 0x00,             // Mapper 0, no special features
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ];
    
    // Add 16KB of PRG ROM (mostly zeros)
    let prg_rom = vec![0; 16384];
    cartridge_data.extend(prg_rom);
    
    // Create cartridge from data
    let cartridge = Cartridge::from_bytes(&cartridge_data)?;
    println!("Test cartridge created");
    
    // Create emulator
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("Testing APU register writes...");
    
    // Test 1: Enable all channels
    println!("Test 1: Enabling all channels...");
    emulator.bus_mut().write_byte(0x4015, 0x1F)?; // Enable all channels
    
    let status = emulator.bus_mut().read_byte(0x4015)?;
    println!("APU Status after enabling: 0x{:02X}", status);
    
    // Test 2: Configure Pulse 1
    println!("Test 2: Configuring Pulse 1...");
    emulator.bus_mut().write_byte(0x4000, 0x3F)?; // Volume 15, constant volume, duty cycle 3
    emulator.bus_mut().write_byte(0x4002, 0xFF)?; // Frequency low
    emulator.bus_mut().write_byte(0x4003, 0x07)?; // Frequency high + length
    
    let status_after_pulse1 = emulator.bus_mut().read_byte(0x4015)?;
    println!("APU Status after Pulse 1 config: 0x{:02X}", status_after_pulse1);
    
    // Test 3: Configure Pulse 2
    println!("Test 3: Configuring Pulse 2...");
    emulator.bus_mut().write_byte(0x4004, 0x3F)?; // Volume 15, constant volume, duty cycle 3
    emulator.bus_mut().write_byte(0x4006, 0x7F)?; // Frequency low
    emulator.bus_mut().write_byte(0x4007, 0x07)?; // Frequency high + length
    
    let status_after_pulse2 = emulator.bus_mut().read_byte(0x4015)?;
    println!("APU Status after Pulse 2 config: 0x{:02X}", status_after_pulse2);
    
    // Test 4: Configure Triangle
    println!("Test 4: Configuring Triangle...");
    emulator.bus_mut().write_byte(0x4008, 0x7F)?; // Linear counter reload
    emulator.bus_mut().write_byte(0x400A, 0xFF)?; // Frequency low
    emulator.bus_mut().write_byte(0x400B, 0x07)?; // Frequency high + length
    
    let status_after_triangle = emulator.bus_mut().read_byte(0x4015)?;
    println!("APU Status after Triangle config: 0x{:02X}", status_after_triangle);
    
    // Test 5: Configure Noise
    println!("Test 5: Configuring Noise...");
    emulator.bus_mut().write_byte(0x400C, 0x3F)?; // Volume 15, constant volume
    emulator.bus_mut().write_byte(0x400E, 0x0F)?; // Period 15
    emulator.bus_mut().write_byte(0x400F, 0x07)?; // Length
    
    let status_after_noise = emulator.bus_mut().read_byte(0x4015)?;
    println!("APU Status after Noise config: 0x{:02X}", status_after_noise);
    
    // Run for a few frames to generate audio
    println!("Running for 10 frames to generate audio...");
    emulator.start();
    
    let mut frame_count = 0;
    let target_frames = 10;
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
    println!("APU register test completed!");
    println!("Total audio samples generated: {}", total_samples);
    
    // Final status check
    let final_status = emulator.bus_mut().read_byte(0x4015)?;
    println!("Final APU Status: 0x{:02X}", final_status);
    
    // Check each channel
    if final_status & 0x01 != 0 {
        println!("✓ Pulse 1 channel is active");
    } else {
        println!("✗ Pulse 1 channel is not active");
    }
    
    if final_status & 0x02 != 0 {
        println!("✓ Pulse 2 channel is active");
    } else {
        println!("✗ Pulse 2 channel is not active");
    }
    
    if final_status & 0x04 != 0 {
        println!("✓ Triangle channel is active");
    } else {
        println!("✗ Triangle channel is not active");
    }
    
    if final_status & 0x08 != 0 {
        println!("✓ Noise channel is active");
    } else {
        println!("✗ Noise channel is not active");
    }
    
    Ok(())
}
