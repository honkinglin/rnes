use rnes_common::{Byte, Word, RnesResult, RnesError, RAM_SIZE, MemoryAccess};
use rnes_cpu6502::Cpu;
use rnes_cartridge::Cartridge;

/// System bus
#[derive(Debug)]
pub struct Bus {
    pub cartridge: Option<Cartridge>,
    pub ram: [Byte; RAM_SIZE],
    pub controller1: rnes_common::ControllerState,
    pub controller2: rnes_common::ControllerState,
}

impl Bus {
    /// Create new bus instance
    pub fn new() -> Self {
        Self {
            cartridge: None,
            ram: [0; RAM_SIZE],
            controller1: rnes_common::ControllerState::default(),
            controller2: rnes_common::ControllerState::default(),
        }
    }
    
    /// Insert cartridge
    pub fn insert_cartridge(&mut self, cartridge: Cartridge) -> RnesResult<()> {
        self.cartridge = Some(cartridge);
        self.reset()?;
        Ok(())
    }
    
    /// Reset system
    pub fn reset(&mut self) -> RnesResult<()> {
        // Reset RAM
        self.ram = [0; RAM_SIZE];
        Ok(())
    }
    
    /// Read byte
    pub fn read_byte(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            // RAM (0x0000-0x1FFF)
            0x0000..=0x1FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                Ok(self.ram[ram_addr])
            }
            
            // PPU registers (0x2000-0x2007)
            0x2000..=0x2007 => {
                // Return 0 for now, will be handled when PPU is implemented
                Ok(0)
            }
            
            // RAM mirroring (0x2008-0x3FFF)
            0x2008..=0x3FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                Ok(self.ram[ram_addr])
            }
            
            // APU and I/O registers (0x4000-0x401F)
            0x4000..=0x401F => {
                match addr {
                    0x4016 => {
                        // Controller 1 state
                        let mut value = 0;
                        if self.controller1.a { value |= 0x01; }
                        if self.controller1.b { value |= 0x02; }
                        if self.controller1.select { value |= 0x04; }
                        if self.controller1.start { value |= 0x08; }
                        if self.controller1.up { value |= 0x10; }
                        if self.controller1.down { value |= 0x20; }
                        if self.controller1.left { value |= 0x40; }
                        if self.controller1.right { value |= 0x80; }
                        Ok(value)
                    }
                    0x4017 => {
                        // Controller 2 state
                        let mut value = 0;
                        if self.controller2.a { value |= 0x01; }
                        if self.controller2.b { value |= 0x02; }
                        if self.controller2.select { value |= 0x04; }
                        if self.controller2.start { value |= 0x08; }
                        if self.controller2.up { value |= 0x10; }
                        if self.controller2.down { value |= 0x20; }
                        if self.controller2.left { value |= 0x40; }
                        if self.controller2.right { value |= 0x80; }
                        Ok(value)
                    }
                    _ => Ok(0), // Other registers return 0 for now
                }
            }
            
            // Cartridge PRG ROM (0x8000-0xFFFF)
            0x8000..=0xFFFF => {
                if let Some(ref cartridge) = self.cartridge {
                    let rom_addr = addr - 0x8000;
                    cartridge.read_prg_rom(rom_addr)
                } else {
                    // Return 0 if no cartridge (for testing)
                    Ok(0)
                }
            }
            
            _ => Err(RnesError::MemoryAccess { address: addr }),
        }
    }
    
    /// Write byte
    pub fn write_byte(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            // RAM (0x0000-0x1FFF)
            0x0000..=0x1FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                self.ram[ram_addr] = value;
                Ok(())
            }
            
            // PPU registers (0x2000-0x2007)
            0x2000..=0x2007 => {
                // Ignore for now, will be handled when PPU is implemented
                tracing::debug!("PPU write: 0x{:04X} = 0x{:02X}", addr, value);
                Ok(())
            }
            
            // RAM mirroring (0x2008-0x3FFF)
            0x2008..=0x3FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                self.ram[ram_addr] = value;
                Ok(())
            }
            
            // APU and I/O registers (0x4000-0x401F)
            0x4000..=0x401F => {
                match addr {
                    0x4016 => {
                        // Controller status register
                        tracing::debug!("Controller status write: 0x{:02X}", value);
                        Ok(())
                    }
                    _ => {
                        // Ignore other registers for now
                        tracing::debug!("I/O write: 0x{:04X} = 0x{:02X}", addr, value);
                        Ok(())
                    }
                }
            }
            
            // Cartridge PRG ROM (0x8000-0xFFFF)
            0x8000..=0xFFFF => {
                if let Some(ref mut cartridge) = self.cartridge {
                    let rom_addr = addr - 0x8000;
                    cartridge.write_prg_rom(rom_addr, value)
                } else {
                    // Ignore write if no cartridge (for testing)
                    Ok(())
                }
            }
            
            _ => Err(RnesError::MemoryAccess { address: addr }),
        }
    }
    
    /// Read word (little-endian)
    pub fn read_word(&self, addr: Word) -> RnesResult<Word> {
        let low = self.read_byte(addr)? as Word;
        let high = self.read_byte(addr + 1)? as Word;
        Ok(low | (high << 8))
    }
    
    /// Write word (little-endian)
    pub fn write_word(&mut self, addr: Word, value: Word) -> RnesResult<()> {
        self.write_byte(addr, value as Byte)?;
        self.write_byte(addr + 1, (value >> 8) as Byte)?;
        Ok(())
    }
    
    /// Set controller 1 state
    pub fn set_controller1(&mut self, state: rnes_common::ControllerState) {
        self.controller1 = state;
    }
    
    /// Set controller 2 state
    pub fn set_controller2(&mut self, state: rnes_common::ControllerState) {
        self.controller2 = state;
    }
    
    /// Execute one CPU cycle (requires CPU instance)
    pub fn step_cpu(&mut self, cpu: &mut Cpu) -> RnesResult<rnes_common::Cycles> {
        cpu.step(self)
    }
}

impl MemoryAccess for Bus {
    fn read_byte(&self, addr: Word) -> RnesResult<Byte> {
        self.read_byte(addr)
    }
    
    fn write_byte(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        self.write_byte(addr, value)
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
