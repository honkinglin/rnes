use rnes_common::{RnesResult, EmulatorState};
use crate::Bus;
use rnes_cartridge::Cartridge;

/// NES Emulator
#[derive(Debug)]
pub struct Emulator {
    pub bus: Bus,
    pub cpu: rnes_cpu6502::Cpu,
    pub state: EmulatorState,
    pub running: bool,
}

impl Emulator {
    /// Create new emulator instance
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cpu: rnes_cpu6502::Cpu::new(),
            state: EmulatorState::default(),
            running: false,
        }
    }
    
    /// Load ROM
    pub fn load_rom(&mut self, cartridge: Cartridge) -> RnesResult<()> {
        self.bus.insert_cartridge(cartridge)?;
        self.reset()?;
        Ok(())
    }
    
    /// Reset emulator
    pub fn reset(&mut self) -> RnesResult<()> {
        self.bus.reset()?;
        self.cpu.reset(&mut self.bus)?;
        
        // Note: Removed the 8-cycle delay as it was causing PC to be modified incorrectly
        
        self.state = EmulatorState::default();
        self.running = false;
        Ok(())
    }
    
    /// Run one CPU cycle
    pub fn step(&mut self) -> RnesResult<rnes_common::Cycles> {
        if !self.running {
            return Ok(0);
        }
        
        let cycles = self.bus.step_cpu(&mut self.cpu)?;
        self.state.cpu_cycles += cycles;
        
        // Update PPU state
        if let Some(ref ppu) = self.bus.ppu {
            self.state.ppu_scanline = ppu.scanline();
            self.state.ppu_dot = ppu.dot();
        }
        
        Ok(cycles)
    }
    
    /// Run specified number of CPU cycles
    pub fn run_cycles(&mut self, cycles: rnes_common::Cycles) -> RnesResult<()> {
        let mut remaining = cycles;
        while remaining > 0 {
            let executed = self.step()?;
            if executed == 0 {
                break;
            }
            remaining = remaining.saturating_sub(executed);
        }
        Ok(())
    }
    
    /// Start running
    pub fn start(&mut self) {
        self.running = true;
    }
    
    /// Stop running
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Get CPU status
    pub fn cpu_status(&self) -> String {
        self.cpu.status_string()
    }
    
    /// Get emulator state
    pub fn get_state(&self) -> &EmulatorState {
        &self.state
    }
    
    /// Get PPU frame buffer
    pub fn get_ppu_frame_buffer(&self) -> Option<&[rnes_common::Pixel]> {
        self.bus.get_ppu_frame_buffer()
    }
    
    /// Check if PPU VBlank is active
    pub fn ppu_vblank(&self) -> bool {
        self.bus.ppu_vblank()
    }
    
    /// Debug: Get PPU registers
    pub fn debug_ppu_registers(&self) -> Option<rnes_ppu::PpuRegisters> {
        self.bus.debug_ppu_registers()
    }
    
    /// Debug: Get PPU state
    pub fn debug_ppu_state(&self) -> Option<&rnes_ppu::PpuState> {
        self.bus.debug_ppu_state()
    }
    
    /// Debug: Check if PPU background is enabled
    pub fn debug_ppu_background_enabled(&self) -> bool {
        self.bus.debug_ppu_background_enabled()
    }
    
    /// Get PPU instance
    pub fn ppu(&self) -> &rnes_ppu::Ppu {
        self.bus.ppu()
    }
    
    /// Get mutable PPU instance
    pub fn ppu_mut(&mut self) -> &mut rnes_ppu::Ppu {
        self.bus.ppu_mut()
    }
    
    /// Get CPU instance
    pub fn cpu(&self) -> &rnes_cpu6502::Cpu {
        &self.cpu
    }
    
    /// Get mutable CPU instance
    pub fn cpu_mut(&mut self) -> &mut rnes_cpu6502::Cpu {
        &mut self.cpu
    }
    
    /// Set controller 1 state
    pub fn set_controller1(&mut self, state: rnes_common::ControllerState) {
        self.bus.set_controller1(state);
    }
    
    /// Set controller 2 state
    pub fn set_controller2(&mut self, state: rnes_common::ControllerState) {
        self.bus.set_controller2(state);
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}
