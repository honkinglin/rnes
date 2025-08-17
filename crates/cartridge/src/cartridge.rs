use std::fs::File;
use std::io::Read;
use std::path::Path;
use rnes_common::{Byte, Word, RnesResult};
use crate::header::{InesHeader, Mirroring};

/// Cartridge implementation
#[derive(Debug)]
pub struct Cartridge {
    pub header: InesHeader,
    pub prg_rom: Vec<Byte>,
    pub chr_rom: Vec<Byte>,
    pub prg_ram: Vec<Byte>,
    pub mirroring: Mirroring,
}

impl Cartridge {
    /// Load ROM from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> RnesResult<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Self::from_bytes(&data)
    }
    
    /// Load ROM from byte array
    pub fn from_bytes(data: &[u8]) -> RnesResult<Self> {
        let header = InesHeader::from_bytes(data)?;
        
        let mut offset = 16; // Header size
        
        // Skip trainer
        if header.has_trainer() {
            offset += 512;
        }
        
        // Read PRG ROM
        let prg_rom_size = header.prg_rom_bytes();
        if offset + prg_rom_size > data.len() {
            return Err(rnes_common::RnesError::RomFormat("Insufficient PRG ROM data".to_string()));
        }
        let prg_rom = data[offset..offset + prg_rom_size].to_vec();
        offset += prg_rom_size;
        
        // Read CHR ROM
        let chr_rom_size = header.chr_rom_bytes();
        let chr_rom = if chr_rom_size > 0 {
            if offset + chr_rom_size > data.len() {
                return Err(rnes_common::RnesError::RomFormat("Insufficient CHR ROM data".to_string()));
            }
            let rom = data[offset..offset + chr_rom_size].to_vec();
            rom
        } else {
            // No CHR ROM, create 8KB CHR RAM
            vec![0; 8192]
        };
        
        // Create PRG RAM
        let prg_ram_size = if header.flags8 > 0 {
            header.flags8 as usize * 8192
        } else {
            8192 // Default 8KB
        };
        let prg_ram = vec![0; prg_ram_size];
        
        let mirroring = header.mirroring();
        Ok(Self {
            header,
            prg_rom,
            chr_rom,
            prg_ram,
            mirroring,
        })
    }
    
    /// Read PRG ROM
    pub fn read_prg_rom(&self, addr: Word) -> RnesResult<Byte> {
        let offset = addr as usize;
        if offset < self.prg_rom.len() {
            Ok(self.prg_rom[offset])
        } else {
            // Mirroring
            let mirrored_offset = offset % self.prg_rom.len();
            Ok(self.prg_rom[mirrored_offset])
        }
    }
    
    /// Write PRG ROM (for some Mappers)
    pub fn write_prg_rom(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        // PRG ROM is usually read-only
        // Some Mappers may need write functionality
        tracing::warn!("Attempting to write to PRG ROM: 0x{:04X} = 0x{:02X}", addr, value);
        Ok(())
    }
    
    /// Read CHR ROM/RAM
    pub fn read_chr(&self, addr: Word) -> RnesResult<Byte> {
        let offset = addr as usize;
        if offset < self.chr_rom.len() {
            Ok(self.chr_rom[offset])
        } else {
            Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Write CHR RAM
    pub fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        let offset = addr as usize;
        if offset < self.chr_rom.len() {
            // If CHR ROM exists, it's usually read-only
            // But some games may use CHR RAM
            tracing::warn!("Attempting to write to CHR ROM: 0x{:04X} = 0x{:02X}", addr, value);
        } else {
            return Err(rnes_common::RnesError::MemoryAccess { address: addr });
        }
        Ok(())
    }
    
    /// Read PRG RAM
    pub fn read_prg_ram(&self, addr: Word) -> RnesResult<Byte> {
        let offset = (addr & 0x1FFF) as usize;
        if offset < self.prg_ram.len() {
            Ok(self.prg_ram[offset])
        } else {
            Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Write PRG RAM
    pub fn write_prg_ram(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        let offset = (addr & 0x1FFF) as usize;
        if offset < self.prg_ram.len() {
            self.prg_ram[offset] = value;
            Ok(())
        } else {
            Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Get mapper number
    pub fn mapper_number(&self) -> u8 {
        self.header.mapper_number()
    }
    
    /// Check if battery backup is present
    pub fn has_battery(&self) -> bool {
        self.header.has_battery()
    }
    
    /// Get mirroring type
    pub fn mirroring(&self) -> Mirroring {
        self.mirroring
    }
}
