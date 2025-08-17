use rnes_ppu::{Ppu, PpuPhase, BackgroundPipeline, SpritePipeline};
use rnes_mappers::NromMapper;
use rnes_cartridge::{Cartridge, Mirroring};
use rnes_common::{Byte, Word, Pixel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PPU Timing Optimization Demo ===");
    
    // Create a test ROM for demonstration
    let test_rom = create_test_rom();
    let cartridge = Cartridge::from_bytes(&test_rom)?;
    let mapper = NromMapper::new(cartridge);
    let mut ppu = Ppu::new(Box::new(mapper));
    
    // Enable rendering
    ppu.write_register(0x2001, 0x18)?; // Enable background and sprites
    
    println!("Initial PPU state:");
    print_ppu_state(&ppu);
    
    // Run PPU for several frames to demonstrate timing optimization
    println!("\nRunning PPU for 3 frames...");
    
    let mut frame_count = 0;
    let mut last_frame = 0;
    
    while frame_count < 3 {
        ppu.step()?;
        
        let current_frame = ppu.frame_count();
        if current_frame > last_frame {
            frame_count += 1;
            last_frame = current_frame;
            
            println!("\n--- Frame {} ---", frame_count);
            print_ppu_state(&ppu);
            
            // Demonstrate timing optimization features
            demonstrate_timing_optimization(&ppu);
        }
    }
    
    // Demonstrate VRAM caching
    println!("\n=== VRAM Cache Demonstration ===");
    demonstrate_vram_caching(&mut ppu)?;
    
    // Demonstrate background pipeline
    println!("\n=== Background Pipeline Demonstration ===");
    demonstrate_background_pipeline(&ppu);
    
    // Demonstrate sprite pipeline
    println!("\n=== Sprite Pipeline Demonstration ===");
    demonstrate_sprite_pipeline(&ppu);
    
    println!("\n=== PPU Timing Optimization Demo Complete ===");
    Ok(())
}

fn create_test_rom() -> Vec<u8> {
    let mut rom_data = vec![
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ];
    
    // Add 16KB PRG ROM data (simple test pattern)
    for i in 0..16384 {
        rom_data.push((i % 256) as u8);
    }
    
    // Add 8KB CHR ROM data (simple tile pattern)
    for i in 0..8192 {
        rom_data.push((i % 64) as u8);
    }
    
    rom_data
}

fn print_ppu_state(ppu: &Ppu) {
    let (frame_count, phase, rendering_enabled, background_enabled) = ppu.timing_stats();
    let (scanline, dot) = (ppu.scanline(), ppu.dot());
    let vblank = ppu.vblank();
    
    println!("  Frame: {}, Scanline: {}, Dot: {}", frame_count, scanline, dot);
    println!("  Phase: {:?}", phase);
    println!("  VBlank: {}", vblank);
    println!("  Rendering enabled: {}", rendering_enabled);
    println!("  Background enabled: {}", background_enabled);
    println!("  Sprites enabled: {}", ppu.sprites_enabled());
}

fn demonstrate_timing_optimization(ppu: &Ppu) {
    let phase = ppu.phase();
    
    match phase {
        PpuPhase::PreRender => {
            println!("  Pre-render scanline: Background rendering and sprite evaluation");
        }
        PpuPhase::Visible => {
            println!("  Visible scanline: Pixel rendering and tile fetching");
            let scanline = ppu.scanline() as usize;
            let dot = ppu.dot() as usize;
            
            if dot >= 1 && dot <= 64 {
                println!("    Sprite evaluation phase (cycles 1-64)");
            } else if dot >= 65 && dot <= 256 {
                println!("    Sprite rendering phase (cycles 65-256)");
            } else if dot >= 257 && dot <= 320 {
                println!("    Background fetching phase (cycles 257-320)");
            }
        }
        PpuPhase::PostRender => {
            println!("  Post-render scanline: Idle phase");
        }
        PpuPhase::VBlank => {
            println!("  VBlank scanline: Vertical blanking period");
        }
    }
}

fn demonstrate_vram_caching(ppu: &mut Ppu) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initial VRAM cache stats:");
    let (valid_entries, total_entries) = ppu.vram_cache_stats();
    println!("  Valid entries: {}/{}", valid_entries, total_entries);
    
    // Simulate some VRAM reads to populate cache
    println!("Simulating VRAM reads...");
    for addr in 0x2000..0x2100 {
        let _ = ppu.read_register(0x2007); // This will trigger VRAM reads
    }
    
    let (valid_entries, total_entries) = ppu.vram_cache_stats();
    println!("After VRAM reads - Valid entries: {}/{}", valid_entries, total_entries);
    
    // Clear cache
    ppu.clear_vram_cache();
    let (valid_entries, total_entries) = ppu.vram_cache_stats();
    println!("After clearing cache - Valid entries: {}/{}", valid_entries, total_entries);
    
    Ok(())
}

fn demonstrate_background_pipeline(ppu: &Ppu) {
    let bg_pipeline = ppu.background_pipeline();
    
    println!("Background Pipeline State:");
    println!("  Nametable latch: 0x{:02X}", bg_pipeline.nametable_latch);
    println!("  Attribute latch: 0x{:02X}", bg_pipeline.attribute_latch);
    println!("  Pattern low latch: 0x{:02X}", bg_pipeline.pattern_low_latch);
    println!("  Pattern high latch: 0x{:02X}", bg_pipeline.pattern_high_latch);
    println!("  Fine X: {}", bg_pipeline.fine_x);
    println!("  Tile counter: {}", bg_pipeline.tile_counter);
    println!("  Fetch phase: {}", bg_pipeline.fetch_phase);
    
    println!("  Shift registers:");
    println!("    Shift high: 0x{:04X}", bg_pipeline.shift_high);
    println!("    Shift low: 0x{:04X}", bg_pipeline.shift_low);
    println!("    Attr shift high: 0x{:04X}", bg_pipeline.attr_shift_high);
    println!("    Attr shift low: 0x{:04X}", bg_pipeline.attr_shift_low);
}

fn demonstrate_sprite_pipeline(ppu: &Ppu) {
    let sprite_pipeline = ppu.sprite_pipeline();
    
    println!("Sprite Pipeline State:");
    println!("  Sprites on scanline: {}", sprite_pipeline.sprites_on_scanline.len());
    println!("  Sprite patterns loaded: {}", sprite_pipeline.sprite_patterns.len());
    println!("  Sprite zero hit: {}", sprite_pipeline.sprite_zero_hit);
    println!("  Sprite overflow: {}", sprite_pipeline.sprite_overflow);
    println!("  Evaluation phase: {}", sprite_pipeline.evaluation_phase);
    println!("  Rendering phase: {}", sprite_pipeline.rendering_phase);
    
    // Show details of sprites on current scanline
    for (i, sprite) in sprite_pipeline.sprites_on_scanline.iter().enumerate() {
        println!("  Sprite {}: Y={}, X={}, Tile={}, Attr=0x{:02X}", 
                i, sprite.y, sprite.x, sprite.tile_id, sprite.attributes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ppu_timing_optimization() {
        let test_rom = create_test_rom();
        let cartridge = Cartridge::from_bytes(&test_rom).unwrap();
        let mapper = NromMapper::new(cartridge);
        let mut ppu = Ppu::new(Box::new(mapper));
        
        // Test initial state
        assert_eq!(ppu.phase(), PpuPhase::PreRender);
        assert_eq!(ppu.frame_count(), 0);
        assert_eq!(ppu.scanline(), -1);
        assert_eq!(ppu.dot(), 0);
        
        // Test timing state updates
        ppu.step().unwrap();
        assert_eq!(ppu.dot(), 1);
        
        // Test rendering flags
        ppu.write_register(0x2001, 0x18).unwrap(); // Enable background and sprites
        assert!(ppu.background_enabled());
        assert!(ppu.sprites_enabled());
        assert!(ppu.rendering_enabled());
    }
    
    #[test]
    fn test_vram_caching() {
        let test_rom = create_test_rom();
        let cartridge = Cartridge::from_bytes(&test_rom).unwrap();
        let mapper = NromMapper::new(cartridge);
        let mut ppu = Ppu::new(Box::new(mapper));
        
        // Test initial cache state
        let (valid_entries, total_entries) = ppu.vram_cache_stats();
        assert_eq!(valid_entries, 0);
        assert_eq!(total_entries, 256);
        
        // Test cache clearing
        ppu.clear_vram_cache();
        let (valid_entries, _) = ppu.vram_cache_stats();
        assert_eq!(valid_entries, 0);
    }
}
