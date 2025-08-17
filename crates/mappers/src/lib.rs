use rnes_common::{Byte, Word, RnesResult};
use rnes_cartridge::Cartridge;

/// Mapper trait for different cartridge types
pub trait Mapper {
    /// Read from PRG ROM/RAM
    fn read_prg(&self, addr: Word) -> RnesResult<Byte>;
    
    /// Write to PRG RAM
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()>;
    
    /// Read from CHR ROM/RAM
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte>;
    
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
    
    /// Get PRG RAM for battery backup
    fn get_prg_ram(&self) -> Option<&[Byte]> {
        None
    }
    
    /// Get mutable PRG RAM for battery backup
    fn get_prg_ram_mut(&mut self) -> Option<&mut [Byte]> {
        None
    }
    
    /// Load PRG RAM from battery backup
    fn load_prg_ram(&mut self, data: &[Byte]) -> RnesResult<()> {
        if let Some(ram) = self.get_prg_ram_mut() {
            if data.len() <= ram.len() {
                ram[..data.len()].copy_from_slice(data);
                Ok(())
            } else {
                Err(rnes_common::RnesError::Serialization("PRG RAM data too large".to_string()))
            }
        } else {
            Err(rnes_common::RnesError::Serialization("No PRG RAM available".to_string()))
        }
    }
    
    /// Check if mapper has battery backup
    fn has_battery(&self) -> bool {
        false
    }
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
    
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte> {
        self.cartridge.read_chr(addr)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        self.cartridge.write_chr(addr, value)
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        self.cartridge.mirroring()
    }
}

/// MMC1 Mapper (Mapper 1)
/// 
/// Features:
/// - 16KB or 32KB PRG ROM banks
/// - 4KB CHR ROM/RAM banks
/// - Battery backup support
/// - Configurable mirroring
/// - Serial shift register for register writes
pub struct Mmc1Mapper {
    cartridge: Cartridge,
    shift_register: u8,
    shift_count: u8,
    control: u8,
    chr_bank0: u8,
    chr_bank1: u8,
    prg_bank: u8,
    prg_ram: Vec<Byte>,
}

impl Mmc1Mapper {
    pub fn new(cartridge: Cartridge) -> Self {
        let prg_ram_size = if cartridge.has_battery() { 8192 } else { 8192 };
        Self {
            cartridge,
            shift_register: 0,
            shift_count: 0,
            control: 0x0C,
            chr_bank0: 0,
            chr_bank1: 0,
            prg_bank: 0,
            prg_ram: vec![0; prg_ram_size],
        }
    }
    
    fn write_register(&mut self, addr: Word, value: Byte) {
        // MMC1 uses a serial shift register
        if value & 0x80 != 0 {
            // Reset shift register
            self.shift_register = 0;
            self.shift_count = 0;
            self.control |= 0x0C;
            return;
        }
        
        // Shift in data
        self.shift_register >>= 1;
        self.shift_register |= (value & 1) << 4;
        self.shift_count += 1;
        
        if self.shift_count == 5 {
            // Register is full, write it
            let register = (addr >> 13) & 3;
            match register {
                0 => self.control = self.shift_register & 0x1F,
                1 => self.chr_bank0 = self.shift_register & 0x1F,
                2 => self.chr_bank1 = self.shift_register & 0x1F,
                3 => self.prg_bank = self.shift_register & 0x0F,
                _ => unreachable!(),
            }
            
            // Reset shift register
            self.shift_register = 0;
            self.shift_count = 0;
        }
    }
    
    fn get_prg_bank(&self, addr: Word) -> Word {
        let bank_mode = (self.control >> 2) & 3;
        let bank = self.prg_bank & 0x0F;
        
        match bank_mode {
            0 | 1 => {
                // 32KB mode
                let bank_32k = (bank >> 1) * 2;
                if addr < 0xC000 {
                    bank_32k.into()
                } else {
                    (bank_32k + 1).into()
                }
            }
            2 => {
                // Fixed first, switchable last
                if addr < 0xC000 {
                    0
                } else {
                    bank.into()
                }
            }
            3 => {
                // Switchable first, fixed last
                if addr < 0xC000 {
                    bank.into()
                } else {
                    (self.cartridge.header.prg_rom_size - 1) as Word
                }
            }
            _ => unreachable!(),
        }
    }
    
    fn get_chr_bank(&self, addr: Word) -> Word {
        let chr_mode = (self.control >> 4) & 1;
        
        if chr_mode == 0 {
            // 8KB mode
            ((self.chr_bank0 >> 1) * 2).into()
        } else {
            // 4KB mode
            if addr < 0x1000 {
                self.chr_bank0.into()
            } else {
                self.chr_bank1.into()
            }
        }
    }
}

impl Mapper for Mmc1Mapper {
    fn read_prg(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x6000..=0x7FFF => {
                // PRG RAM
                let ram_addr = (addr - 0x6000) as usize;
                if ram_addr < self.prg_ram.len() {
                    Ok(self.prg_ram[ram_addr])
                } else {
                    Err(rnes_common::RnesError::MemoryAccess { address: addr })
                }
            }
            0x8000..=0xFFFF => {
                // PRG ROM
                let bank = self.get_prg_bank(addr);
                let bank_offset = (bank as usize) * 16384;
                let rom_addr = bank_offset + ((addr - 0x8000) as usize);
                self.cartridge.read_prg_rom(rom_addr as Word)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x6000..=0x7FFF => {
                // PRG RAM
                let ram_addr = (addr - 0x6000) as usize;
                if ram_addr < self.prg_ram.len() {
                    self.prg_ram[ram_addr] = value;
                    Ok(())
                } else {
                    Err(rnes_common::RnesError::MemoryAccess { address: addr })
                }
            }
            0x8000..=0xFFFF => {
                // Register write
                self.write_register(addr, value);
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte> {
        let bank = self.get_chr_bank(addr);
        let bank_offset = (bank as usize) * 4096;
        let chr_addr = bank_offset + (addr as usize);
        self.cartridge.read_chr(chr_addr as Word)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        // MMC1 can have CHR RAM
        let bank = self.get_chr_bank(addr);
        let bank_offset = (bank as usize) * 4096;
        let chr_addr = bank_offset + (addr as usize);
        self.cartridge.write_chr(chr_addr as Word, value)
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        match self.control & 3 {
            0 => rnes_cartridge::Mirroring::SingleScreenA,
            1 => rnes_cartridge::Mirroring::SingleScreenB,
            2 => rnes_cartridge::Mirroring::Vertical,
            3 => rnes_cartridge::Mirroring::Horizontal,
            _ => unreachable!(),
        }
    }
    
    fn get_prg_ram(&self) -> Option<&[Byte]> {
        Some(&self.prg_ram)
    }
    
    fn get_prg_ram_mut(&mut self) -> Option<&mut [Byte]> {
        Some(&mut self.prg_ram)
    }
    
    fn has_battery(&self) -> bool {
        self.cartridge.has_battery()
    }
}

/// UxROM Mapper (Mapper 2)
/// 
/// Features:
/// - 16KB PRG ROM banks (switchable)
/// - Fixed last 16KB of PRG ROM
/// - 8KB CHR ROM/RAM
/// - Simple bank switching
pub struct UxromMapper {
    cartridge: Cartridge,
    prg_bank: u8,
}

impl UxromMapper {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            prg_bank: 0,
        }
    }
}

impl Mapper for UxromMapper {
    fn read_prg(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x8000..=0xBFFF => {
                // Switchable 16KB bank
                let bank_offset = (self.prg_bank as usize) * 16384;
                let rom_addr = bank_offset + ((addr - 0x8000) as usize);
                self.cartridge.read_prg_rom(rom_addr as Word)
            }
            0xC000..=0xFFFF => {
                // Fixed last 16KB
                let rom_addr = addr - 0x8000;
                self.cartridge.read_prg_rom(rom_addr)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x8000..=0xFFFF => {
                // Bank select
                self.prg_bank = value & 0x0F;
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte> {
        self.cartridge.read_chr(addr)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        self.cartridge.write_chr(addr, value)
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        self.cartridge.mirroring()
    }
}

/// CNROM Mapper (Mapper 3)
/// 
/// Features:
/// - 32KB PRG ROM (fixed)
/// - 8KB CHR ROM banks (switchable)
/// - Simple CHR bank switching
pub struct CnromMapper {
    cartridge: Cartridge,
    chr_bank: u8,
}

impl CnromMapper {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            chr_bank: 0,
        }
    }
}

impl Mapper for CnromMapper {
    fn read_prg(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x8000..=0xFFFF => {
                // Fixed 32KB PRG ROM
                let rom_addr = addr - 0x8000;
                self.cartridge.read_prg_rom(rom_addr)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x8000..=0xFFFF => {
                // CHR bank select
                self.chr_bank = value & 0x03;
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte> {
        let bank_offset = (self.chr_bank as usize) * 8192;
        let chr_addr = bank_offset + (addr as usize);
        self.cartridge.read_chr(chr_addr as Word)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        // CNROM typically has CHR ROM, so writes are ignored
        tracing::warn!("Attempting to write to CHR ROM: 0x{:04X} = 0x{:02X}", addr, value);
        Ok(())
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        self.cartridge.mirroring()
    }
}

/// AOROM Mapper (Mapper 7)
/// 
/// Features:
/// - 32KB PRG ROM banks (switchable)
/// - 8KB CHR ROM/RAM
/// - Simple bank switching
/// - Configurable mirroring
pub struct AoromMapper {
    cartridge: Cartridge,
    prg_bank: u8,
    mirroring: rnes_cartridge::Mirroring,
}

impl AoromMapper {
    pub fn new(cartridge: Cartridge) -> Self {
        let mirroring = cartridge.mirroring();
        Self {
            cartridge,
            prg_bank: 0,
            mirroring,
        }
    }
}

impl Mapper for AoromMapper {
    fn read_prg(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x8000..=0xFFFF => {
                // Switchable 32KB bank
                let bank_offset = (self.prg_bank as usize) * 32768;
                let rom_addr = bank_offset + ((addr - 0x8000) as usize);
                self.cartridge.read_prg_rom(rom_addr as Word)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x8000..=0xFFFF => {
                // Bank select and mirroring
                self.prg_bank = value & 0x0F;
                self.mirroring = if value & 0x10 != 0 {
                    rnes_cartridge::Mirroring::SingleScreenA
                } else {
                    rnes_cartridge::Mirroring::SingleScreenB
                };
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte> {
        self.cartridge.read_chr(addr)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        self.cartridge.write_chr(addr, value)
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        self.mirroring
    }
}

/// MMC3 Mapper (Mapper 4)
/// 
/// Features:
/// - 8KB PRG ROM banks (switchable)
/// - 1KB CHR ROM/RAM banks (switchable)
/// - Scanline IRQ counter
/// - Battery backup support
/// - Configurable mirroring
/// - A12 clock detection for IRQ
pub struct Mmc3Mapper {
    cartridge: Cartridge,
    prg_ram: Vec<Byte>,
    
    // Bank registers
    bank_select: u8,
    bank_data: [u8; 8],
    
    // Mirroring control
    mirroring: rnes_cartridge::Mirroring,
    
    // IRQ counter
    irq_counter: u8,
    irq_latch: u8,
    irq_enabled: bool,
    irq_pending: bool,
    
    // A12 clock detection
    last_a12: bool,
    a12_rising_edge: bool,
}

impl Mmc3Mapper {
    pub fn new(cartridge: Cartridge) -> Self {
        let prg_ram_size = if cartridge.has_battery() { 8192 } else { 8192 };
        let mirroring = cartridge.mirroring();
        Self {
            cartridge,
            prg_ram: vec![0; prg_ram_size],
            bank_select: 0,
            bank_data: [0; 8],
            mirroring,
            irq_counter: 0,
            irq_latch: 0,
            irq_enabled: false,
            irq_pending: false,
            last_a12: false,
            a12_rising_edge: false,
        }
    }
    
    fn write_bank_select(&mut self, value: Byte) {
        self.bank_select = value;
    }
    
    fn write_bank_data(&mut self, value: Byte) {
        let bank = (self.bank_select & 0x07) as usize;
        self.bank_data[bank] = value;
    }
    
    fn write_mirroring(&mut self, value: Byte) {
        self.mirroring = if value & 0x01 != 0 {
            rnes_cartridge::Mirroring::Horizontal
        } else {
            rnes_cartridge::Mirroring::Vertical
        };
    }
    
    fn write_irq_latch(&mut self, value: Byte) {
        self.irq_latch = value;
    }
    
    fn write_irq_reload(&mut self, _value: Byte) {
        self.irq_counter = self.irq_latch;
    }
    
    fn write_irq_disable(&mut self, _value: Byte) {
        self.irq_enabled = false;
        self.irq_pending = false;
    }
    
    fn write_irq_enable(&mut self, _value: Byte) {
        self.irq_enabled = true;
    }
    
    fn get_prg_bank(&self, addr: Word) -> Word {
        let bank_mode = (self.bank_select >> 6) & 1;
        let bank_num = (addr >> 13) & 1;
        
        match (bank_mode, bank_num) {
            (0, 0) => {
                // Fixed first bank, switchable second bank
                if addr < 0xC000 {
                    0 // Fixed first 8KB
                } else {
                    (self.bank_data[6] & 0x3F) as Word // Switchable 8KB
                }
            }
            (0, 1) => {
                // Switchable first bank, fixed last bank
                if addr < 0xC000 {
                    (self.bank_data[6] & 0x3F) as Word // Switchable 8KB
                } else {
                    (self.cartridge.header.prg_rom_size - 1) as Word // Fixed last 8KB
                }
            }
            (1, 0) => {
                // Switchable first bank, fixed last bank
                if addr < 0xC000 {
                    (self.bank_data[7] & 0x3F) as Word // Switchable 8KB
                } else {
                    (self.cartridge.header.prg_rom_size - 1) as Word // Fixed last 8KB
                }
            }
            (1, 1) => {
                // Fixed first bank, switchable second bank
                if addr < 0xC000 {
                    0 // Fixed first 8KB
                } else {
                    (self.bank_data[7] & 0x3F) as Word // Switchable 8KB
                }
            }
            _ => unreachable!(),
        }
    }
    
    fn get_chr_bank(&self, addr: Word) -> Word {
        let chr_mode = (self.bank_select >> 7) & 1;
        let bank_num = (addr >> 10) & 3;
        
        match (chr_mode, bank_num) {
            (0, 0) => {
                // 2KB mode for first bank
                if addr < 0x0800 {
                    (self.bank_data[0] & 0xFE) as Word // 2KB bank
                } else if addr < 0x1000 {
                    (self.bank_data[1] & 0xFE) as Word // 2KB bank
                } else {
                    (self.bank_data[2] as Word) * 2 // 1KB bank
                }
            }
            (0, 1) => {
                // 2KB mode for second bank
                if addr < 0x0800 {
                    (self.bank_data[2] as Word) * 2 // 1KB bank
                } else if addr < 0x1000 {
                    (self.bank_data[3] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[4] as Word) * 2 // 1KB bank
                }
            }
            (0, 2) => {
                // 2KB mode for third bank
                if addr < 0x0800 {
                    (self.bank_data[4] as Word) * 2 // 1KB bank
                } else if addr < 0x1000 {
                    (self.bank_data[5] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[0] & 0xFE) as Word // 2KB bank
                }
            }
            (0, 3) => {
                // 2KB mode for fourth bank
                if addr < 0x0800 {
                    (self.bank_data[6] as Word) * 2 // 1KB bank
                } else if addr < 0x1000 {
                    (self.bank_data[7] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[1] & 0xFE) as Word // 2KB bank
                }
            }
            (1, 0) => {
                // 1KB mode for first bank
                if addr < 0x0400 {
                    (self.bank_data[0] as Word) * 2 // 1KB bank
                } else if addr < 0x0800 {
                    (self.bank_data[1] as Word) * 2 // 1KB bank
                } else if addr < 0x0C00 {
                    (self.bank_data[2] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[3] as Word) * 2 // 1KB bank
                }
            }
            (1, 1) => {
                // 1KB mode for second bank
                if addr < 0x0400 {
                    (self.bank_data[4] as Word) * 2 // 1KB bank
                } else if addr < 0x0800 {
                    (self.bank_data[5] as Word) * 2 // 1KB bank
                } else if addr < 0x0C00 {
                    (self.bank_data[6] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[7] as Word) * 2 // 1KB bank
                }
            }
            (1, 2) => {
                // 1KB mode for third bank
                if addr < 0x0400 {
                    (self.bank_data[0] as Word) * 2 // 1KB bank
                } else if addr < 0x0800 {
                    (self.bank_data[1] as Word) * 2 // 1KB bank
                } else if addr < 0x0C00 {
                    (self.bank_data[2] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[3] as Word) * 2 // 1KB bank
                }
            }
            (1, 3) => {
                // 1KB mode for fourth bank
                if addr < 0x0400 {
                    (self.bank_data[4] as Word) * 2 // 1KB bank
                } else if addr < 0x0800 {
                    (self.bank_data[5] as Word) * 2 // 1KB bank
                } else if addr < 0x0C00 {
                    (self.bank_data[6] as Word) * 2 // 1KB bank
                } else {
                    (self.bank_data[7] as Word) * 2 // 1KB bank
                }
            }
            _ => unreachable!(),
        }
    }
    
    fn clock_irq_counter(&mut self) {
        if self.irq_counter == 0 {
            self.irq_counter = self.irq_latch;
        } else {
            self.irq_counter -= 1;
        }
        
        if self.irq_counter == 0 && self.irq_enabled {
            self.irq_pending = true;
        }
    }
    
    fn detect_a12_rising_edge(&mut self, addr: Word) {
        let current_a12 = (addr & 0x1000) != 0;
        self.a12_rising_edge = !self.last_a12 && current_a12;
        self.last_a12 = current_a12;
        
        // If we detected a rising edge, clock the IRQ counter
        if self.a12_rising_edge {
            self.clock_irq_counter();
        }
    }
}

impl Mapper for Mmc3Mapper {
    fn read_prg(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x6000..=0x7FFF => {
                // PRG RAM
                let ram_addr = (addr - 0x6000) as usize;
                if ram_addr < self.prg_ram.len() {
                    Ok(self.prg_ram[ram_addr])
                } else {
                    Err(rnes_common::RnesError::MemoryAccess { address: addr })
                }
            }
            0x8000..=0xFFFF => {
                // PRG ROM
                let bank = self.get_prg_bank(addr);
                let bank_offset = (bank as usize) * 8192;
                let rom_addr = bank_offset + ((addr & 0x1FFF) as usize);
                self.cartridge.read_prg_rom(rom_addr as Word)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn write_prg(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x6000..=0x7FFF => {
                // PRG RAM
                let ram_addr = (addr - 0x6000) as usize;
                if ram_addr < self.prg_ram.len() {
                    self.prg_ram[ram_addr] = value;
                    Ok(())
                } else {
                    Err(rnes_common::RnesError::MemoryAccess { address: addr })
                }
            }
            0x8000..=0x9FFF => {
                if addr & 1 == 0 {
                    // Even address: Bank select
                    self.write_bank_select(value);
                } else {
                    // Odd address: Bank data
                    self.write_bank_data(value);
                }
                Ok(())
            }
            0xA000..=0xBFFF => {
                if addr & 1 == 0 {
                    // Even address: Mirroring
                    self.write_mirroring(value);
                } else {
                    // Odd address: PRG RAM protect
                    // Ignored for now
                }
                Ok(())
            }
            0xC000..=0xDFFF => {
                if addr & 1 == 0 {
                    // Even address: IRQ latch
                    self.write_irq_latch(value);
                } else {
                    // Odd address: IRQ reload
                    self.write_irq_reload(value);
                }
                Ok(())
            }
            0xE000..=0xFFFF => {
                if addr & 1 == 0 {
                    // Even address: IRQ disable
                    self.write_irq_disable(value);
                } else {
                    // Odd address: IRQ enable
                    self.write_irq_enable(value);
                }
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    fn read_chr(&mut self, addr: Word) -> RnesResult<Byte> {
        // Detect A12 rising edge for IRQ counter
        self.detect_a12_rising_edge(addr);
        
        let bank = self.get_chr_bank(addr);
        let bank_offset = (bank as usize) * 1024;
        let chr_addr = bank_offset + ((addr & 0x03FF) as usize);
        self.cartridge.read_chr(chr_addr as Word)
    }
    
    fn write_chr(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        // MMC3 can have CHR RAM
        let bank = self.get_chr_bank(addr);
        let bank_offset = (bank as usize) * 1024;
        let chr_addr = bank_offset + ((addr & 0x03FF) as usize);
        self.cartridge.write_chr(chr_addr as Word, value)
    }
    
    fn mirroring(&self) -> rnes_cartridge::Mirroring {
        self.mirroring
    }
    
    fn irq_pending(&self) -> bool {
        self.irq_pending
    }
    
    fn clear_irq(&mut self) {
        self.irq_pending = false;
    }
    
    fn step(&mut self) {
        // MMC3 IRQ counter is clocked by A12 rising edge
        // The counter is now handled in detect_a12_rising_edge
        self.a12_rising_edge = false;
    }
    
    fn get_prg_ram(&self) -> Option<&[Byte]> {
        Some(&self.prg_ram)
    }
    
    fn get_prg_ram_mut(&mut self) -> Option<&mut [Byte]> {
        Some(&mut self.prg_ram)
    }
    
    fn has_battery(&self) -> bool {
        self.cartridge.has_battery()
    }
}

/// Create mapper from cartridge
pub fn create_mapper(cartridge: Cartridge) -> RnesResult<Box<dyn Mapper>> {
    match cartridge.mapper_number() {
        0 => Ok(Box::new(NromMapper::new(cartridge))),
        1 => Ok(Box::new(Mmc1Mapper::new(cartridge))),
        2 => Ok(Box::new(UxromMapper::new(cartridge))),
        3 => Ok(Box::new(CnromMapper::new(cartridge))),
        7 => Ok(Box::new(AoromMapper::new(cartridge))),
        4 => Ok(Box::new(Mmc3Mapper::new(cartridge))),
        mapper => Err(rnes_common::RnesError::UnsupportedMapper(mapper))
    }
}
