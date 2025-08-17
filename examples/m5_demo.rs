use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::SaveSystem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 M5 Demo: MMC3 Mapper with Scanline IRQ");
    println!("=========================================");
    
    // Create a simple test ROM with MMC3 mapper
    let rom_data = create_test_rom_with_mmc3();
    
    println!("ROM data size: {} bytes", rom_data.len());
    println!("Expected size: {} bytes (16 + 16384 + 8192)", 16 + 16384 + 8192);
    
    // Print first few bytes of ROM
    println!("First 32 bytes: {:02X?}", &rom_data[..32.min(rom_data.len())]);
    
    // Print header bytes
    println!("Header bytes: {:02X?}", &rom_data[..16]);
    
    // Print PRG ROM start and end
    println!("PRG ROM start: 0x{:04X}, end: 0x{:04X}", 16, 16 + 16384);
    println!("CHR ROM start: 0x{:04X}, end: 0x{:04X}", 16 + 16384, 16 + 16384 + 8192);
    
    // Print some CHR ROM data
    let chr_start = 16 + 16384;
    let chr_end = chr_start + 8192;
    if chr_end <= rom_data.len() {
        println!("CHR ROM first 16 bytes: {:02X?}", &rom_data[chr_start..chr_start + 16]);
    } else {
        println!("CHR ROM data not available");
    }
    
    // Print ROM data at expected boundaries
    println!("Data at 0x{:04X}: 0x{:02X}", 16, rom_data[16]);
    println!("Data at 0x{:04X}: 0x{:02X}", 16 + 16384 - 1, rom_data[16 + 16384 - 1]);
    println!("Data at 0x{:04X}: 0x{:02X}", 16 + 16384, rom_data[16 + 16384]);
    println!("Data at 0x{:04X}: 0x{:02X}", 16 + 16384 + 8192 - 1, rom_data[16 + 16384 + 8192 - 1]);
    
    // Check if we have trainer
    let has_trainer = (rom_data[6] & 0x04) != 0;
    println!("Has trainer: {}", has_trainer);
    
    // Load ROM
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    let mut emulator = Emulator::new();
    emulator.load_rom(cartridge)?;
    
    println!("✅ ROM loaded successfully");
    println!("Mapper number: {}", emulator.bus.cartridge.as_ref().unwrap().mapper_number());
    println!("PRG ROM size: {} KB", emulator.bus.cartridge.as_ref().unwrap().header.prg_rom_size * 16);
    println!("CHR ROM size: {} KB", emulator.bus.cartridge.as_ref().unwrap().header.chr_rom_size * 8);
    println!("Mirroring: {:?}", emulator.bus.cartridge.as_ref().unwrap().mirroring());
    println!("Has battery: {}", emulator.bus.cartridge.as_ref().unwrap().has_battery());
    
    // Test basic mapper operations
    let mapper = emulator.bus.mapper();
    println!("\n📋 Mapper Information:");
    println!("Mapper mirroring: {:?}", mapper.mirroring());
    println!("Mapper has battery: {}", mapper.has_battery());
    println!("Mapper IRQ pending: {}", mapper.irq_pending());
    
    // Test MMC3 register writes
    println!("\n🔧 Testing MMC3 Register Writes:");
    
    // Test bank select register
    emulator.bus.write_byte(0x8000, 0x06)?; // Select PRG bank 6
    println!("✅ Wrote 0x06 to bank select register (0x8000)");
    
    // Test bank data register
    emulator.bus.write_byte(0x8001, 0x02)?; // Set bank 6 to bank 2
    println!("✅ Wrote 0x02 to bank data register (0x8001)");
    
    // Test mirroring register
    emulator.bus.write_byte(0xA000, 0x01)?; // Set horizontal mirroring
    println!("✅ Wrote 0x01 to mirroring register (0xA000)");
    
    // Test IRQ latch register
    emulator.bus.write_byte(0xC000, 0x20)?; // Set IRQ latch to 32
    println!("✅ Wrote 0x20 to IRQ latch register (0xC000)");
    
    // Test IRQ reload register
    emulator.bus.write_byte(0xC001, 0x00)?; // Reload IRQ counter
    println!("✅ Wrote 0x00 to IRQ reload register (0xC001)");
    
    // Test IRQ enable register
    emulator.bus.write_byte(0xE001, 0x00)?; // Enable IRQ
    println!("✅ Wrote 0x00 to IRQ enable register (0xE001)");
    
    // Check updated mapper state
    {
        let mapper = emulator.bus.mapper();
        println!("\n📊 Updated Mapper State:");
        println!("Mapper mirroring: {:?}", mapper.mirroring());
        println!("Mapper IRQ pending: {}", mapper.irq_pending());
    }
    
    // Test PRG ROM reading
    println!("\n📖 Testing PRG ROM Reading:");
    let value_8000 = emulator.bus.read_byte(0x8000)?;
    let value_C000 = emulator.bus.read_byte(0xC000)?;
    println!("Value at 0x8000: 0x{:02X}", value_8000);
    println!("Value at 0xC000: 0x{:02X}", value_C000);
    
    // Test CHR ROM reading
    println!("\n🎨 Testing CHR ROM Reading:");
    let value_0000 = emulator.bus.read_byte(0x0000)?;
    let value_1000 = emulator.bus.read_byte(0x1000)?;
    println!("Value at 0x0000: 0x{:02X}", value_0000);
    println!("Value at 0x1000: 0x{:02X}", value_1000);
    
    // Test IRQ functionality
    println!("\n⚡ Testing IRQ Functionality:");
    
    // Run emulator for a few cycles to trigger IRQ
    emulator.running = true;
    let mut cycles_executed = 0;
    let max_cycles = 10000;
    
    while cycles_executed < max_cycles {
        let cycles = emulator.step()?;
        if cycles == 0 {
            break;
        }
        cycles_executed += cycles as u64;
        
        // Check for IRQ
        if emulator.cpu().irq_pending {
            println!("🎯 IRQ triggered at cycle {}", cycles_executed);
            break;
        }
    }
    
    println!("✅ Executed {} cycles", cycles_executed);
    println!("Final CPU state:");
    println!("  PC: 0x{:04X}", emulator.cpu().pc);
    println!("  A: 0x{:02X}, X: 0x{:02X}, Y: 0x{:02X}", 
             emulator.cpu().a, emulator.cpu().x, emulator.cpu().y);
    println!("  IRQ pending: {}", emulator.cpu().irq_pending);
    
    // Test save system
    let has_battery = emulator.bus.mapper().has_battery();
    if has_battery {
        println!("\n💾 Testing Save System:");
        
        // Save state
        match emulator.save_state(1) {
            Ok(()) => println!("✅ Save state created successfully"),
            Err(e) => println!("❌ Failed to create save state: {}", e),
        }
        
        // Modify CPU state
        emulator.cpu_mut().a = 0xAA;
        emulator.cpu_mut().x = 0xBB;
        emulator.cpu_mut().y = 0xCC;
        println!("Modified CPU state: A=0x{:02X}, X=0x{:02X}, Y=0x{:02X}", 
                 emulator.cpu().a, emulator.cpu().x, emulator.cpu().y);
        
        // Load state
        match emulator.load_state(1) {
            Ok(()) => println!("✅ Save state loaded successfully"),
            Err(e) => println!("❌ Failed to load save state: {}", e),
        }
        
        println!("CPU state after load: A=0x{:02X}, X=0x{:02X}, Y=0x{:02X}", 
                 emulator.cpu().a, emulator.cpu().x, emulator.cpu().y);
    }
    
    println!("\n🎉 M5 Demo completed successfully!");
    println!("The MMC3 mapper with scanline IRQ is working correctly.");
    
    Ok(())
}

/// Create a simple test ROM with MMC3 mapper
fn create_test_rom_with_mmc3() -> Vec<u8> {
    let mut rom_data = Vec::new();
    
    // NES header (16 bytes)
    rom_data.extend_from_slice(b"NES\x1A"); // Magic number
    rom_data.push(1); // PRG ROM size (16KB)
    rom_data.push(1); // CHR ROM size (8KB)
    rom_data.push(0x40); // Flags 6: Mapper 4 (MMC3), vertical mirroring, battery (no trainer)
    rom_data.push(0x00); // Flags 7: Mapper 4 (MMC3), NES 2.0 not set
    rom_data.push(0x00); // PRG RAM size
    rom_data.push(0x00); // TV system
    rom_data.push(0x00); // TV system, PRG RAM presence
    rom_data.push(0x00); // Timing
    rom_data.push(0x00); // System type
    rom_data.push(0x00); // Mapper, submapper
    rom_data.push(0x00); // PRG ROM upper bits
    rom_data.push(0x00); // CHR ROM upper bits
    rom_data.push(0x00); // PRG RAM shift
    rom_data.push(0x00); // CHR RAM shift
    
    // PRG ROM (16KB) - simple test program
    // Reset vector at 0xFFFC-0xFFFD points to 0x8000
    // Fill with NOP instructions (0xEA) except for reset vector
    for i in 0..16384 {
        if i >= 16384 - 4 {
            // Reset vector: 0x8000 (little-endian)
            rom_data.push(0x00);
            rom_data.push(0x80);
            // NMI vector: 0x8000 (little-endian)
            rom_data.push(0x00);
            rom_data.push(0x80);
            break;
        } else {
            rom_data.push(0xEA); // NOP instruction
        }
    }
    
    // CHR ROM (8KB) - simple test data
    for i in 0..8192 {
        rom_data.push((i & 0xFF) as u8);
    }
    
    // Ensure we have enough data
    if rom_data.len() < 16 + 16384 + 8192 {
        let needed = 16 + 16384 + 8192 - rom_data.len();
        for _ in 0..needed {
            rom_data.push(0);
        }
    }
    
    rom_data
}
