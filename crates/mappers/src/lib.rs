use rnes_common::{Byte, Word, RnesResult};
use rnes_cartridge::Cartridge;

/// Mapper trait for different cartridge types
pub trait Mapper {
    /// Read from PRG ROM/RAM
    fn read_prg(&self, addr: Word) -> RnesResult<Byte>;
    
    /// Write to PRG RAM
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()>;
    
    /// Read from CHR ROM/RAM
    fn read_chr(&self, addr: Word) -> RnesResult<Byte>;
    
    /// Write to CHR RAM
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()>;
    
    /// Get mirroring type
    fn mirroring(&self) -> rnes_cartridge::Mirroring;
    
    /// Check if IRQ is pending
    fn irq_pending(&self) -> bool {
        false
    }
    
    /// Clear IRQ
    fn clear_irq(&mut self) {}
    
    /// Step mapper (for mappers with internal state)
    fn step(&mut self) {}
}

/// NROM Mapper (Mapper 0)
/// 
/// Memory mapping:
/// - PRG ROM: 0x8000-0xFFFF (32KB or 16KB mirrored)
/// - CHR ROM/RAM: 0x0000-0x1FFF (8KB)
/// - PRG RAM: 0x6000-0x7FFF (8KB, if present)
pub struct NromMapper {
    cartridge: Cartridge,
}

impl NromMapper {
    pub fn new(cartridge: Cartridge) -> Self {
        Self { cartridge }
    }
}

impl Mapper for NromMapper {
    fn read_prg(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x6000..=0x7FFF => {
                // PRG RAM
                self.cartridge.read_prg_ram(addr - 0x6000)
            }
            0x8000..=0xFFFF => {
                // PRG ROM
                let rom_addr = addr - 0x8000;
                self.cartridge.read_prg_rom(rom_addr)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x6000..=0x7FFF => {
                // PRG RAM
                self.cartridge.write_prg_ram(addr - 0x6000, value)
            }
            0x8000..=0xFFFF => {
                // PRG ROM is read-only
                tracing::warn!("Attempting to write to PRG ROM: 0x{:04X} = 0x{:02X}", addr, value);
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn read_chr(&self, addr: Word) -> RnesResult<Byte> {
        self.cartridge.read_chr(addr)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        self.cartridge.write_chr(addr, value)
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        self.cartridge.mirroring()
    }
}

/// Create mapper from cartridge
pub fn create_mapper(cartridge: Cartridge) -> RnesResult<Box<dyn Mapper>> {
    match cartridge.mapper_number() {
        0 => Ok(Box::new(NromMapper::new(cartridge))),
        mapper => Err(rnes_common::RnesError::UnsupportedMapper(mapper))
    }
}
