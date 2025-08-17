use rnes_cartridge::Cartridge;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 RNES ROM Analyzer");
    println!("===================");
    println!("Analyzing real NES ROM program content");
    println!("");

    let test_roms = vec![
        "tests/roms/ppu-tests/nrom368/test1.nes",
        "tests/roms/ppu-tests/full_palette/full_palette.nes",
        "tests/roms/ppu-tests/blargg_ppu_tests/vbl_clear_time.nes",
    ];

    for rom_path in test_roms {
        if Path::new(rom_path).exists() {
            println!("📂 Analyzing: {}", rom_path);
            println!("{}", "-".repeat(50));
            
            analyze_rom(rom_path)?;
            println!("");
        } else {
            println!("⏭️  SKIP: {} (file not found)", rom_path);
        }
    }

    Ok(())
}

fn analyze_rom(rom_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cartridge = Cartridge::from_file(rom_path)?;
    
    println!("  Mapper: {}", cartridge.mapper_number());
    println!("  Mirroring: {:?}", cartridge.mirroring());
    println!("  PRG ROM: {} bytes", cartridge.prg_rom.len());
    println!("  CHR ROM: {} bytes", cartridge.chr_rom.len());
    
    // Analyze reset vector
    let reset_low = cartridge.prg_rom[0x3FFC];
    let reset_high = cartridge.prg_rom[0x3FFD];
    let reset_addr = ((reset_high as u16) << 8) | (reset_low as u16);
    println!("  Reset vector: 0x{:04X}", reset_addr);
    
    // Convert to PRG ROM offset
    let prg_offset = if reset_addr >= 0x8000 {
        reset_addr - 0x8000
    } else {
        reset_addr
    };
    
    println!("  PRG ROM offset: 0x{:04X}", prg_offset);
    
    // Show first 32 bytes of program
    println!("  First 32 bytes of program:");
    for i in 0..32 {
        if (prg_offset as usize + i) < cartridge.prg_rom.len() {
            let byte = cartridge.prg_rom[prg_offset as usize + i];
            print!("  {:02X}", byte);
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
    }
    println!();
    
    // Look for PPU register writes
    println!("  Searching for PPU register writes...");
    let mut ppu_writes = Vec::new();
    
    for i in 0..cartridge.prg_rom.len().saturating_sub(2) {
        let byte1 = cartridge.prg_rom[i];
        let byte2 = cartridge.prg_rom[i + 1];
        let byte3 = cartridge.prg_rom[i + 2];
        
        // Look for STA $20xx (store to PPU registers)
        if byte1 == 0x8D && byte2 <= 0x07 && byte3 == 0x20 {
            ppu_writes.push((i, byte2, byte3));
        }
        
        // Look for STA $20xx (store to PPU registers) - different addressing
        if byte1 == 0x9D && byte2 <= 0x07 && byte3 == 0x20 {
            ppu_writes.push((i, byte2, byte3));
        }
    }
    
    if ppu_writes.is_empty() {
        println!("  No PPU register writes found");
    } else {
        println!("  Found {} PPU register writes:", ppu_writes.len());
        for (offset, reg, _) in ppu_writes.iter().take(10) {
            let reg_name = match reg {
                0x00 => "PPUCTRL",
                0x01 => "PPUMASK", 
                0x02 => "PPUSTATUS",
                0x03 => "OAMADDR",
                0x04 => "OAMDATA",
                0x05 => "PPUSCROLL",
                0x06 => "PPUADDR",
                0x07 => "PPUDATA",
                _ => "Unknown"
            };
            println!("    Offset 0x{:04X}: Write to {} (0x20{:02X})", offset, reg_name, reg);
        }
    }
    
    Ok(())
}
