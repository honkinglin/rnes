use rnes_common::{Byte, Word, RnesResult, RnesError, RAM_SIZE, MemoryAccess};
use rnes_cpu6502::Cpu;
use rnes_cartridge::Cartridge;
use rnes_ppu::Ppu;
use rnes_apu::Apu;

/// System bus
pub struct Bus {
    pub cartridge: Option<Cartridge>,
    pub ppu: Option<Ppu>,
    pub apu: Option<Apu>,
    pub ram: [Byte; RAM_SIZE],
    pub controller1: rnes_common::ControllerState,
    pub controller2: rnes_common::ControllerState,
}

impl Bus {
    /// Create new bus instance
    pub fn new() -> Self {
        Self {
            cartridge: None,
            ppu: None,
            apu: None,
            ram: [0; RAM_SIZE],
            controller1: rnes_common::ControllerState::default(),
            controller2: rnes_common::ControllerState::default(),
        }
    }
    
    /// Insert cartridge
    pub fn insert_cartridge(&mut self, cartridge: Cartridge) -> RnesResult<()> {
        self.cartridge = Some(cartridge.clone());
        
        // Create mapper, PPU, and APU
        let mapper = rnes_mappers::create_mapper(cartridge)?;
        let ppu = Ppu::new(mapper);
        let apu = Apu::new();
        
        self.ppu = Some(ppu);
        self.apu = Some(apu);
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
    pub fn read_byte(&mut self, addr: Word) -> RnesResult<Byte> {
        match addr {
            // RAM (0x0000-0x1FFF)
            0x0000..=0x1FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                Ok(self.ram[ram_addr])
            }
            
            // PPU registers (0x2000-0x2007)
            0x2000..=0x2007 => {
                if let Some(ref mut ppu) = self.ppu {
                    ppu.read_register(addr)
                } else {
                    Ok(0)
                }
            }
            
            // RAM mirroring (0x2008-0x3FFF)
            0x2008..=0x3FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                Ok(self.ram[ram_addr])
            }
            
            // APU and I/O registers (0x4000-0x401F)
            0x4000..=0x401F => {
                match addr {
                    0x4015 => {
                        // APU status
                        if let Some(ref apu) = self.apu {
                            apu.read_register(addr)
                        } else {
                            Ok(0)
                        }
                    }
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
                    _ => {
                        // APU registers
                        if let Some(ref apu) = self.apu {
                            apu.read_register(addr)
                        } else {
                            Ok(0)
                        }
                    }
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
                if let Some(ref mut ppu) = self.ppu {
                    ppu.write_register(addr, value)
                } else {
                    tracing::debug!("PPU write: 0x{:04X} = 0x{:02X}", addr, value);
                    Ok(())
                }
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
                    0x4014 => {
                        // OAM DMA
                        if let Some(ref mut ppu) = self.ppu {
                            ppu.start_oam_dma(value);
                        }
                        Ok(())
                    }
                    0x4015 => {
                        // APU status
                        if let Some(ref mut apu) = self.apu {
                            apu.write_register(addr, value)
                        } else {
                            Ok(())
                        }
                    }
                    0x4016 => {
                        // Controller status register
                        tracing::debug!("Controller status write: 0x{:02X}", value);
                        Ok(())
                    }
                    0x4017 => {
                        // Frame counter
                        if let Some(ref mut apu) = self.apu {
                            apu.write_register(addr, value)
                        } else {
                            Ok(())
                        }
                    }
                    _ => {
                        // APU registers
                        if let Some(ref mut apu) = self.apu {
                            apu.write_register(addr, value)
                        } else {
                            tracing::debug!("APU write: 0x{:04X} = 0x{:02X}", addr, value);
                            Ok(())
                        }
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
        let cycles = cpu.step(self)?;
        
        // Step PPU (3x CPU clock)
        if let Some(ref mut ppu) = self.ppu {
            for _ in 0..cycles * 3 {
                ppu.step()?;
            }
        }
        
        // Step APU (1x CPU clock)
        if let Some(ref mut apu) = self.apu {
            for _ in 0..cycles {
                apu.step()?;
            }
        }
        
        Ok(cycles)
    }
    
    /// Get PPU frame buffer
    pub fn get_ppu_frame_buffer(&self) -> Option<&[rnes_common::Pixel]> {
        self.ppu.as_ref().map(|ppu| ppu.frame_buffer())
    }
    
    /// Check if PPU VBlank is active
    pub fn ppu_vblank(&self) -> bool {
        self.ppu.as_ref().map(|ppu| ppu.vblank()).unwrap_or(false)
    }
    
    /// Debug: Get PPU registers
    pub fn debug_ppu_registers(&self) -> Option<rnes_ppu::PpuRegisters> {
        self.ppu.as_ref().map(|ppu| ppu.debug_registers())
    }
    
    /// Debug: Get PPU state
    pub fn debug_ppu_state(&self) -> Option<&rnes_ppu::PpuState> {
        self.ppu.as_ref().map(|ppu| ppu.debug_state())
    }
    
    /// Debug: Check if PPU background is enabled
    pub fn debug_ppu_background_enabled(&self) -> bool {
        self.ppu.as_ref().map(|ppu| ppu.debug_background_enabled()).unwrap_or(false)
    }
    
    /// Get PPU instance
    pub fn ppu(&self) -> &rnes_ppu::Ppu {
        self.ppu.as_ref().expect("PPU not initialized")
    }
    
    /// Get mutable PPU instance
    pub fn ppu_mut(&mut self) -> &mut rnes_ppu::Ppu {
        self.ppu.as_mut().expect("PPU not initialized")
    }
    
    /// Get APU instance
    pub fn apu(&self) -> &rnes_apu::Apu {
        self.apu.as_ref().expect("APU not initialized")
    }
    
    /// Get mutable APU instance
    pub fn apu_mut(&mut self) -> &mut rnes_apu::Apu {
        self.apu.as_mut().expect("APU not initialized")
    }
    
    /// Get mapper instance
    pub fn mapper(&self) -> &dyn rnes_mappers::Mapper {
        self.ppu.as_ref().expect("PPU not initialized").mapper()
    }
    
    /// Get mutable mapper instance
    pub fn mapper_mut(&mut self) -> &mut dyn rnes_mappers::Mapper {
        self.ppu.as_mut().expect("PPU not initialized").mapper_mut()
    }
    
    /// Get audio samples from APU
    pub fn get_audio_samples(&mut self) -> Vec<rnes_common::AudioSample> {
        self.apu.as_mut()
            .map(|apu| apu.get_samples())
            .unwrap_or_default()
    }
    
    /// Check if DMC IRQ is pending
    pub fn dmc_irq_pending(&self) -> bool {
        self.apu.as_ref()
            .map(|apu| apu.dmc_irq_pending())
            .unwrap_or(false)
    }
    
    /// Clear DMC IRQ
    pub fn clear_dmc_irq(&mut self) {
        if let Some(ref mut apu) = self.apu {
            apu.clear_dmc_irq();
        }
    }
}

impl MemoryAccess for Bus {
    fn read_byte(&self, addr: Word) -> RnesResult<Byte> {
        // For now, we'll need to restructure this to avoid the mutable requirement
        // This is a temporary workaround
        match addr {
            0x0000..=0x1FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                Ok(self.ram[ram_addr])
            }
            0x2000..=0x2007 => {
                // PPU registers - return 0 for now to avoid mutable issues
                // In a real implementation, this would need to be handled differently
                match addr {
                    0x2002 => {
                        // PPUSTATUS - return a value that indicates VBlank is not set
                        Ok(0x00)
                    }
                    _ => Ok(0)
                }
            }
            0x2008..=0x3FFF => {
                let ram_addr = (addr & 0x07FF) as usize;
                Ok(self.ram[ram_addr])
            }
            0x8000..=0xFFFF => {
                if let Some(ref cartridge) = self.cartridge {
                    let rom_addr = addr - 0x8000;
                    cartridge.read_prg_rom(rom_addr)
                } else {
                    Ok(0)
                }
            }
            _ => Err(RnesError::MemoryAccess { address: addr })
        }
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

impl std::fmt::Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bus")
            .field("cartridge", &self.cartridge.is_some())
            .field("ppu", &self.ppu.is_some())
            .field("ram", &"[...]")
            .field("controller1", &self.controller1)
            .field("controller2", &self.controller2)
            .finish()
    }
}
