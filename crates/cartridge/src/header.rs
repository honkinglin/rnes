use rnes_common::{Byte, RnesResult, RnesError};

/// iNES ROM header
#[derive(Debug, Clone)]
pub struct InesHeader {
    pub magic: [u8; 4],      // "NES\x1A"
    pub prg_rom_size: Byte,  // PRG ROM size (16KB units)
    pub chr_rom_size: Byte,  // CHR ROM size (8KB units)
    pub flags6: Byte,        // Mapper, mirroring, battery, trainer
    pub flags7: Byte,        // Mapper, VS/Playchoice, NES 2.0
    pub flags8: Byte,        // PRG-RAM size (8KB units)
    pub flags9: Byte,        // TV system
    pub flags10: Byte,       // TV system, PRG-RAM presence
    pub padding: [u8; 5],    // Unused
}

impl InesHeader {
    /// Parse header from byte array
    pub fn from_bytes(data: &[u8]) -> RnesResult<Self> {
        if data.len() < 16 {
            return Err(RnesError::RomFormat("ROM data too small".to_string()));
        }
        
        let magic = [data[0], data[1], data[2], data[3]];
        if magic != [0x4E, 0x45, 0x53, 0x1A] {
            return Err(RnesError::RomFormat("Invalid iNES magic number".to_string()));
        }
        
        Ok(Self {
            magic,
            prg_rom_size: data[4],
            chr_rom_size: data[5],
            flags6: data[6],
            flags7: data[7],
            flags8: data[8],
            flags9: data[9],
            flags10: data[10],
            padding: [data[11], data[12], data[13], data[14], data[15]],
        })
    }
    
    /// Get mapper number
    pub fn mapper_number(&self) -> u8 {
        (self.flags6 >> 4) | (self.flags7 & 0xF0)
    }
    
    /// Check if battery backup is present
    pub fn has_battery(&self) -> bool {
        (self.flags6 & 0x02) != 0
    }
    
    /// Check if trainer is present
    pub fn has_trainer(&self) -> bool {
        (self.flags6 & 0x04) != 0
    }
    
    /// Get mirroring type
    pub fn mirroring(&self) -> Mirroring {
        match self.flags6 & 0x09 {
            0x00 => Mirroring::Horizontal,
            0x01 => Mirroring::Vertical,
            0x08 => Mirroring::FourScreen,
            _ => Mirroring::Horizontal, // Default
        }
    }
    
    /// Check if this is NES 2.0 format
    pub fn is_nes2(&self) -> bool {
        (self.flags7 & 0x0C) == 0x08
    }
    
    /// Get PRG ROM size in bytes
    pub fn prg_rom_bytes(&self) -> usize {
        self.prg_rom_size as usize * 16384
    }
    
    /// Get CHR ROM size in bytes
    pub fn chr_rom_bytes(&self) -> usize {
        self.chr_rom_size as usize * 8192
    }
    
    /// Get trainer size in bytes
    pub fn trainer_bytes(&self) -> usize {
        if self.has_trainer() { 512 } else { 0 }
    }
}

/// Mirroring type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
    SingleScreenA,
    SingleScreenB,
}
