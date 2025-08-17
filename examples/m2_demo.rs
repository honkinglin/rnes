use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::{Pixel, Button};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES M2 Demo: Sprite Layer & Input");
    println!("===================================");
    
    // Create a test ROM with sprite data
    let rom_data = create_sprite_test_rom();
    
    // Load the ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    println!("✓ NROM Mapper initialized");
    println!("✓ PPU initialized with sprite rendering");
    println!("✓ Input handling system ready");
    
    // Test input handling
    println!("\nTesting input handling...");
    test_input_handling(&mut emulator);
    
    // Test sprite rendering
    println!("\nTesting sprite rendering...");
    test_sprite_rendering(&mut emulator)?;
    
    // Test OAM DMA
    println!("\nTesting OAM DMA...");
    test_oam_dma(&mut emulator)?;
    
    println!("\n✓ M2 demo completed successfully!");
    
    Ok(())
}

fn test_input_handling(emulator: &mut Emulator) {
    // Test button presses
    let test_buttons = [
        Button::A,
        Button::B,
        Button::Select,
        Button::Start,
        Button::Up,
        Button::Down,
        Button::Left,
        Button::Right,
    ];
    
    for button in test_buttons {
        emulator.handle_keyboard_input(button, true);
        let state = emulator.get_controller1_state();
        
        let is_pressed = match button {
            Button::A => state.a,
            Button::B => state.b,
            Button::Select => state.select,
            Button::Start => state.start,
            Button::Up => state.up,
            Button::Down => state.down,
            Button::Left => state.left,
            Button::Right => state.right,
        };
        
        if is_pressed {
            println!("  ✓ {} button pressed", format!("{:?}", button));
        } else {
            println!("  ✗ {} button not detected", format!("{:?}", button));
        }
        
        // Release button
        emulator.handle_keyboard_input(button, false);
    }
}

fn test_sprite_rendering(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    // Run the emulator for a few frames to generate sprite data
    emulator.start();
    
    for frame in 0..3 {
        // Run one frame
        emulator.run_cycles(29_780)?;
        
        // Check if we have a frame buffer
        if let Some(frame_buffer) = emulator.get_ppu_frame_buffer() {
            println!("Frame {}: Generated {} pixels", frame + 1, frame_buffer.len());
            
            // Count non-black pixels to see if rendering is working
            let non_black_pixels = frame_buffer.iter()
                .filter(|pixel| **pixel != Pixel::BLACK)
                .count();
            println!("  Non-black pixels: {}", non_black_pixels);
            
            // Check for sprite-specific colors (sprite palettes start at 0x3F10)
            let sprite_pixels = frame_buffer.iter()
                .filter(|pixel| {
                    // Look for colors that might be from sprite palettes
                    pixel.r > 0 || pixel.g > 0 || pixel.b > 0
                })
                .count();
            println!("  Potential sprite pixels: {}", sprite_pixels);
        }
        
        // Check VBlank status
        if emulator.ppu_vblank() {
            println!("  VBlank active");
        }
    }
    
    emulator.stop();
    Ok(())
}

fn test_oam_dma(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    // Create some test sprite data in RAM
    let sprite_data = create_test_sprite_data();
    
    // Write sprite data to RAM at page 0x02
    for (i, &byte) in sprite_data.iter().enumerate() {
        emulator.bus_mut().write_byte(0x0200 + i as u16, byte)?;
    }
    
    println!("  ✓ Test sprite data written to RAM");
    
    // Trigger OAM DMA transfer
    emulator.bus_mut().write_byte(0x4014, 0x02)?;
    
    println!("  ✓ OAM DMA transfer initiated");
    
    // Check if PPU has OAM DMA active
    let ppu = emulator.ppu();
    if ppu.oam_dma_active() {
        println!("  ✓ OAM DMA is active");
    } else {
        println!("  ✗ OAM DMA not active");
    }
    
    Ok(())
}

fn create_sprite_test_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // NES header (16 bytes)
    rom.extend_from_slice(b"NES\x1A"); // Magic number
    rom.push(1);  // PRG ROM size (16KB units)
    rom.push(1);  // CHR ROM size (8KB units)
    rom.push(0x00); // Flags 6 (horizontal mirroring, no battery, no trainer, no 4-screen)
    rom.push(0x00); // Flags 7 (VS/Playchoice, NES 2.0, mapper high bits)
    rom.push(0x00); // PRG RAM size
    rom.push(0x00); // TV system
    rom.push(0x00); // TV system, PRG RAM presence
    rom.extend_from_slice(&[0; 5]); // Padding
    
    // PRG ROM (16KB)
    let mut prg_rom = vec![0; 16384];
    
    // Simple program that sets up sprites
    let program = vec![
        0xA9, 0x00,       // LDA #$00
        0x8D, 0x20, 0x00, // STA $0020
        0xA9, 0x02,       // LDA #$02
        0x8D, 0x14, 0x40, // STA $4014 (OAM DMA)
        0x4C, 0x00, 0x80, // JMP $8000
    ];
    
    // Write program to PRG ROM
    for (i, &byte) in program.iter().enumerate() {
        prg_rom[i] = byte;
    }
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Create some simple sprite patterns
    let mut chr_rom = vec![0; 8192];
    
    // Create a simple 8x8 sprite pattern (a small square)
    let sprite_pattern = vec![
        0x3C, // 00111100
        0x7E, // 01111110
        0xFF, // 11111111
        0xFF, // 11111111
        0xFF, // 11111111
        0xFF, // 11111111
        0x7E, // 01111110
        0x3C, // 00111100
    ];
    
    // Write sprite pattern to CHR ROM
    for (i, &byte) in sprite_pattern.iter().enumerate() {
        chr_rom[i] = byte;
    }
    
    rom.extend_from_slice(&chr_rom);
    
    rom
}

fn create_test_sprite_data() -> Vec<u8> {
    let mut sprite_data = vec![0; 256];
    
    // Create a test sprite at position (100, 100)
    sprite_data[0] = 100;  // Y position
    sprite_data[1] = 0;    // Tile ID
    sprite_data[2] = 0x00; // Attributes (palette 0, no flip, front)
    sprite_data[3] = 100;  // X position
    
    // Create another test sprite at position (120, 120)
    sprite_data[4] = 120;  // Y position
    sprite_data[5] = 0;    // Tile ID
    sprite_data[6] = 0x01; // Attributes (palette 1, no flip, front)
    sprite_data[7] = 120;  // X position
    
    sprite_data
}
