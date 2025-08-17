use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::MemoryAccess;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RNES Reset Vector Test");
    println!("=====================");
    
    // Load test ROM
    let rom_path = "tests/roms/ppu-tests/nrom368/test1.nes";
    let rom_data = std::fs::read(rom_path)?;
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    
    println!("ROM loaded successfully");
    println!("  PRG ROM size: {} bytes", cartridge.prg_rom.len());
    println!("  CHR ROM size: {} bytes", cartridge.chr_rom.len());
    
    // Check reset vector location
    let reset_vector_low = cartridge.read_prg_rom(0x3FFC)?;
    let reset_vector_high = cartridge.read_prg_rom(0x3FFD)?;
    let reset_vector = (reset_vector_high as u16) << 8 | (reset_vector_low as u16);
    
    println!("  Reset vector: 0x{:04X} (low: 0x{:02X}, high: 0x{:02X})", 
             reset_vector, reset_vector_low, reset_vector_high);
    
    // Create a simple test memory that returns 0x0000 for reset vector
    struct TestMemory {
        reset_vector: u16,
    }
    
    impl MemoryAccess for TestMemory {
        fn read_byte(&self, addr: u16) -> rnes_common::RnesResult<rnes_common::Byte> {
            if addr == 0xFFFC {
                Ok(self.reset_vector as u8)
            } else if addr == 0xFFFD {
                Ok((self.reset_vector >> 8) as u8)
            } else {
                Ok(0)
            }
        }
        
        fn write_byte(&mut self, _addr: u16, _value: rnes_common::Byte) -> rnes_common::RnesResult<()> {
            Ok(())
        }
        
        fn read_word(&self, addr: u16) -> rnes_common::RnesResult<u16> {
            if addr == 0xFFFC {
                Ok(self.reset_vector)
            } else {
                Ok(0)
            }
        }
        
        fn write_word(&mut self, _addr: u16, _value: u16) -> rnes_common::RnesResult<()> {
            Ok(())
        }
    }
    
    // Test CPU reset with 0x0000 reset vector
    let mut test_memory = TestMemory { reset_vector: 0x0000 };
    let mut emulator = Emulator::new();
    emulator.cpu.reset(&mut test_memory)?;
    
    println!("\nCPU reset with 0x0000 reset vector:");
    println!("  PC: 0x{:04X}", emulator.cpu.pc);
    println!("  Expected: 0x8000");
    
    // Test CPU reset with 0x8000 reset vector
    let mut test_memory2 = TestMemory { reset_vector: 0x8000 };
    let mut emulator2 = Emulator::new();
    emulator2.cpu.reset(&mut test_memory2)?;
    
    println!("\nCPU reset with 0x8000 reset vector:");
    println!("  PC: 0x{:04X}", emulator2.cpu.pc);
    println!("  Expected: 0x8000");
    
    Ok(())
}
