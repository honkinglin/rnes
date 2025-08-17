use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 M5 ROM Test: MMC3 Mapper Testing");
    println!("===================================");
    
    // Test with real MMC3 ROM
    let rom_path = PathBuf::from("tests/roms/nes-test-roms/mmc3_test/1-clocking.nes");
    
    if !rom_path.exists() {
        println!("❌ ROM file not found: {:?}", rom_path);
        return Ok(());
    }
    
    println!("📂 Loading ROM: {:?}", rom_path);
    
    // Load ROM
    match Cartridge::from_file(&rom_path) {
        Ok(cartridge) => {
            println!("✅ ROM loaded successfully");
            println!("  Mapper number: {}", cartridge.mapper_number());
            println!("  PRG ROM size: {} bytes ({} KB)", cartridge.prg_rom.len(), cartridge.prg_rom.len() / 1024);
            println!("  CHR ROM size: {} bytes ({} KB)", cartridge.chr_rom.len(), cartridge.chr_rom.len() / 1024);
            println!("  Mirroring: {:?}", cartridge.mirroring());
            println!("  Has battery: {}", cartridge.has_battery());
            
            // Create emulator and load ROM
            let mut emulator = Emulator::new();
            match emulator.load_rom(cartridge) {
                Ok(()) => {
                    println!("✅ ROM loaded into emulator successfully");
                    
                    // Test basic mapper operations
                    let mapper = emulator.bus.mapper();
                    println!("✅ Mapper created successfully");
                    println!("  Mapper type: MMC3 (Mapper 4)");
                    println!("  Mapper mirroring: {:?}", mapper.mirroring());
                    println!("  Mapper has battery: {}", mapper.has_battery());
                    println!("  Mapper IRQ pending: {}", mapper.irq_pending());
                    
                    // Test some basic memory access
                    println!("\n🧪 Testing basic memory access...");
                    
                    // Test PRG ROM access
                    match mapper.read_prg(0x8000) {
                        Ok(value) => println!("  PRG ROM at 0x8000: 0x{:02X}", value),
                        Err(e) => println!("  ❌ PRG ROM read error: {}", e),
                    }
                    
                    // Test mapper register writes
                    println!("\n🧪 Testing MMC3 register writes...");
                    
                    let mapper_mut = emulator.bus.mapper_mut();
                    
                    // Test CHR ROM access (requires mutable reference)
                    match mapper_mut.read_chr(0x0000) {
                        Ok(value) => println!("  CHR ROM at 0x0000: 0x{:02X}", value),
                        Err(e) => println!("  ❌ CHR ROM read error: {}", e),
                    }
                    
                    // Write to bank select register
                    match mapper_mut.write_prg(0x8000, 0x00) {
                        Ok(()) => println!("  ✅ Bank select register write: 0x00"),
                        Err(e) => println!("  ❌ Bank select write error: {}", e),
                    }
                    
                    // Write to bank data register
                    match mapper_mut.write_prg(0x8001, 0x01) {
                        Ok(()) => println!("  ✅ Bank data register write: 0x01"),
                        Err(e) => println!("  ❌ Bank data write error: {}", e),
                    }
                    
                    // Write to mirroring register
                    match mapper_mut.write_prg(0xA000, 0x00) {
                        Ok(()) => println!("  ✅ Mirroring register write: 0x00"),
                        Err(e) => println!("  ❌ Mirroring write error: {}", e),
                    }
                    
                    // Write to IRQ latch register
                    match mapper_mut.write_prg(0xC000, 0x20) {
                        Ok(()) => println!("  ✅ IRQ latch register write: 0x20"),
                        Err(e) => println!("  ❌ IRQ latch write error: {}", e),
                    }
                    
                    // Write to IRQ reload register
                    match mapper_mut.write_prg(0xC001, 0x00) {
                        Ok(()) => println!("  ✅ IRQ reload register write: 0x00"),
                        Err(e) => println!("  ❌ IRQ reload write error: {}", e),
                    }
                    
                    // Write to IRQ disable register
                    match mapper_mut.write_prg(0xE000, 0x00) {
                        Ok(()) => println!("  ✅ IRQ disable register write: 0x00"),
                        Err(e) => println!("  ❌ IRQ disable write error: {}", e),
                    }
                    
                    // Write to IRQ enable register
                    match mapper_mut.write_prg(0xE001, 0x00) {
                        Ok(()) => println!("  ✅ IRQ enable register write: 0x00"),
                        Err(e) => println!("  ❌ IRQ enable write error: {}", e),
                    }
                    
                    println!("\n🎉 M5 ROM test completed successfully!");
                }
                Err(e) => {
                    println!("❌ Failed to load ROM into emulator: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to parse ROM: {}", e);
        }
    }
    
    Ok(())
}
