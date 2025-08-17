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
    
    /// Handle keyboard input
    pub fn handle_keyboard_input(&mut self, key: rnes_common::Button, pressed: bool) {
        let mut controller1 = self.bus.controller1.clone();
        
        match key {
            rnes_common::Button::A => controller1.a = pressed,
            rnes_common::Button::B => controller1.b = pressed,
            rnes_common::Button::Select => controller1.select = pressed,
            rnes_common::Button::Start => controller1.start = pressed,
            rnes_common::Button::Up => controller1.up = pressed,
            rnes_common::Button::Down => controller1.down = pressed,
            rnes_common::Button::Left => controller1.left = pressed,
            rnes_common::Button::Right => controller1.right = pressed,
        }
        
        self.bus.set_controller1(controller1);
    }
    
    /// Get current controller 1 state
    pub fn get_controller1_state(&self) -> &rnes_common::ControllerState {
        &self.bus.controller1
    }
    
    /// Get current controller 2 state
    pub fn get_controller2_state(&self) -> &rnes_common::ControllerState {
        &self.bus.controller2
    }
    
    /// Get mutable bus instance
    pub fn bus_mut(&mut self) -> &mut crate::Bus {
        &mut self.bus
    }
    
    /// Get bus instance
    pub fn bus(&self) -> &crate::Bus {
        &self.bus
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}
