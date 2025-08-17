use rnes_common::{RnesResult, EmulatorState, SaveSystem, SaveState, Config, Debugger, DebugInfo, CpuRegisters, StatusFlagsDebug, PpuDebugState, PpuRegistersDebug, MemoryAccess};
use crate::Bus;
use rnes_cartridge::Cartridge;

/// NES Emulator
#[derive(Debug)]
pub struct Emulator {
    pub bus: Bus,
    pub cpu: rnes_cpu6502::Cpu,
    pub state: EmulatorState,
    pub running: bool,
    pub save_system: SaveSystem,
    pub rom_name: Option<String>,
    pub config: Config,
    pub debugger: Debugger,
    pub auto_save_timer: u32,
    pub last_auto_save: std::time::Instant,
}

impl Emulator {
    /// Create new emulator instance
    pub fn new() -> Self {
        let config = Config::load_or_create().unwrap_or_else(|_| Config::default());
        Self {
            bus: Bus::new(),
            cpu: rnes_cpu6502::Cpu::new(),
            state: EmulatorState::default(),
            running: false,
            save_system: SaveSystem::new(),
            rom_name: None,
            config,
            debugger: Debugger::new(),
            auto_save_timer: 0,
            last_auto_save: std::time::Instant::now(),
        }
    }
    
    /// Create emulator with custom configuration
    pub fn with_config(config: Config) -> Self {
        Self {
            bus: Bus::new(),
            cpu: rnes_cpu6502::Cpu::new(),
            state: EmulatorState::default(),
            running: false,
            save_system: SaveSystem::new(),
            rom_name: None,
            config,
            debugger: Debugger::new(),
            auto_save_timer: 0,
            last_auto_save: std::time::Instant::now(),
        }
    }
    
    /// Load ROM
    pub fn load_rom(&mut self, cartridge: Cartridge) -> RnesResult<()> {
        // Extract ROM name from cartridge
        let rom_name = cartridge.header.magic.iter().map(|&b| b as char).collect::<String>();
        self.rom_name = Some(rom_name.clone());
        
        self.bus.insert_cartridge(cartridge)?;
        
        // Load battery backup if available
        let mapper = self.bus.mapper_mut();
        if mapper.has_battery() {
            if let Ok(data) = self.save_system.load_battery_backup(&rom_name) {
                if !data.is_empty() {
                    mapper.load_prg_ram(&data)?;
                    tracing::info!("Loaded battery backup for ROM: {}", rom_name);
                }
            }
        }
        
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
        self.auto_save_timer = 0;
        self.last_auto_save = std::time::Instant::now();
        
        // Clear debugger state
        self.debugger.clear_history();
        
        Ok(())
    }
    
    /// Run one CPU cycle
    pub fn step(&mut self) -> RnesResult<rnes_common::Cycles> {
        if !self.running {
            return Ok(0);
        }
        
        // Check for breakpoints
        if self.debugger.should_break(self.cpu.pc) {
            self.running = false;
            self.debugger.break_next = false;
            tracing::info!("Breakpoint hit at 0x{:04X}", self.cpu.pc);
            return Ok(0);
        }
        
        let cycles = self.bus.step_cpu(&mut self.cpu)?;
        self.state.cpu_cycles += cycles;
        
        // Update debug info after execution
        self.update_debug_info();
        
        // Update PPU state
        if let Some(ref ppu) = self.bus.ppu {
            self.state.ppu_scanline = ppu.scanline();
            self.state.ppu_dot = ppu.dot();
        }
        
        // Handle APU IRQ
        if self.bus.dmc_irq_pending() {
            self.cpu.request_irq();
            self.bus.clear_dmc_irq();
        }
        
        // Handle auto-save
        self.handle_auto_save()?;
        
        Ok(cycles)
    }
    
    /// Update debug information
    fn update_debug_info(&mut self) {
        let mut debug_info = DebugInfo::default();
        
        // CPU registers
        debug_info.cpu_registers = CpuRegisters {
            a: self.cpu.a,
            x: self.cpu.x,
            y: self.cpu.y,
            sp: self.cpu.sp,
            pc: self.cpu.pc,
            status: self.cpu.status.bits(),
            status_flags: StatusFlagsDebug {
                carry: self.cpu.status.contains(rnes_cpu6502::StatusFlags::CARRY),
                zero: self.cpu.status.contains(rnes_cpu6502::StatusFlags::ZERO),
                interrupt_disable: self.cpu.status.contains(rnes_cpu6502::StatusFlags::INTERRUPT_DISABLE),
                decimal: self.cpu.status.contains(rnes_cpu6502::StatusFlags::DECIMAL),
                r#break: self.cpu.status.contains(rnes_cpu6502::StatusFlags::BREAK),
                overflow: self.cpu.status.contains(rnes_cpu6502::StatusFlags::OVERFLOW),
                negative: self.cpu.status.contains(rnes_cpu6502::StatusFlags::NEGATIVE),
            },
        };
        
        // Current instruction info (simplified)
        debug_info.current_pc = self.cpu.pc;
        debug_info.current_cycles = 0; // This will be updated by the debugger
        debug_info.total_cycles = self.state.cpu_cycles as u64;
        
        // PPU state
        if let Some(ref ppu) = self.bus.ppu {
            debug_info.ppu_state = PpuDebugState {
                scanline: ppu.scanline(),
                dot: ppu.dot(),
                frame: ppu.frame_count(),
                vblank: ppu.vblank(),
                sprite_overflow: ppu.debug_state().sprite_overflow,
                sprite_zero_hit: ppu.debug_state().sprite_zero_hit,
                registers: PpuRegistersDebug {
                    ppuctrl: ppu.debug_registers().ppuctrl,
                    ppumask: ppu.debug_registers().ppumask,
                    ppustatus: ppu.debug_registers().ppustatus,
                    oamaddr: ppu.debug_registers().oamaddr,
                    oamdata: ppu.debug_registers().oamdata,
                    ppuscroll: ppu.debug_registers().ppuscroll,
                    ppuaddr: ppu.debug_registers().ppuaddr,
                    ppudata: ppu.debug_registers().ppudata,
                },
            };
        }
        
        self.debugger.update_debug_info(debug_info);
    }
    
    /// Handle auto-save functionality
    fn handle_auto_save(&mut self) -> RnesResult<()> {
        if !self.config.general.auto_save_battery || self.config.general.auto_save_interval == 0 {
            return Ok(());
        }
        
        let elapsed = self.last_auto_save.elapsed().as_secs() as u32;
        if elapsed >= self.config.general.auto_save_interval {
            self.save_battery_backup()?;
            self.last_auto_save = std::time::Instant::now();
        }
        
        Ok(())
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
    
    /// Get audio samples
    pub fn get_audio_samples(&mut self) -> Vec<rnes_common::AudioSample> {
        self.bus.get_audio_samples()
    }
    
    /// Get APU instance
    pub fn apu(&self) -> &rnes_apu::Apu {
        self.bus.apu()
    }
    
    /// Get mutable APU instance
    pub fn apu_mut(&mut self) -> &mut rnes_apu::Apu {
        self.bus.apu_mut()
    }
    
    /// Save battery backup
    pub fn save_battery_backup(&self) -> RnesResult<()> {
        if let Some(ref rom_name) = self.rom_name {
            let mapper = self.bus.mapper();
            if mapper.has_battery() {
                if let Some(ram) = mapper.get_prg_ram() {
                    self.save_system.save_battery_backup(rom_name, ram)?;
                }
            }
        }
        Ok(())
    }
    
    /// Save state to slot
    pub fn save_state(&self, slot: u8) -> RnesResult<()> {
        if let Some(ref rom_name) = self.rom_name {
            let mut save_state = SaveState::new(rom_name.clone());
            
            // Save CPU state
            save_state.cpu_state.pc = self.cpu.pc;
            save_state.cpu_state.sp = self.cpu.sp;
            save_state.cpu_state.a = self.cpu.a;
            save_state.cpu_state.x = self.cpu.x;
            save_state.cpu_state.y = self.cpu.y;
            save_state.cpu_state.status = self.cpu.status.bits();
            save_state.cpu_state.cycles = self.state.cpu_cycles as u64;
            
            // Save PPU state
            if let Some(ref ppu) = self.bus.ppu {
                save_state.ppu_state.scanline = ppu.scanline() as u16;
                save_state.ppu_state.dot = ppu.dot() as u16;
                save_state.ppu_state.frame = ppu.frame_count() as u32;
                save_state.ppu_state.vblank = ppu.vblank();
                save_state.ppu_state.oam = ppu.oam().to_vec();
                save_state.ppu_state.palette_ram = ppu.palette_ram().to_vec();
                
                // Convert frame buffer to u32 for serialization
                let frame_buffer = ppu.frame_buffer();
                save_state.ppu_state.frame_buffer = frame_buffer.iter()
                    .map(|pixel| pixel.to_u32())
                    .collect();
            }
            
            // Save memory state
            save_state.memory_state.ram = self.bus.ram.to_vec();
            let mapper = self.bus.mapper();
            if let Some(ram) = mapper.get_prg_ram() {
                save_state.memory_state.prg_ram = ram.to_vec();
            }
            
            // Save mapper state
            save_state.mapper_state.mapper_type = self.bus.cartridge.as_ref()
                .map(|c| c.mapper_number())
                .unwrap_or(0);
            // Note: Mapper-specific state serialization would go here
            
            save_state.save_to_file(&self.save_system, slot)?;
        }
        Ok(())
    }
    
    /// Load state from slot
    pub fn load_state(&mut self, slot: u8) -> RnesResult<()> {
        if let Some(ref rom_name) = self.rom_name {
            let save_state = SaveState::load_from_file(&self.save_system, rom_name, slot)?;
            
            // Load CPU state
            self.cpu.pc = save_state.cpu_state.pc;
            self.cpu.sp = save_state.cpu_state.sp;
            self.cpu.a = save_state.cpu_state.a;
            self.cpu.x = save_state.cpu_state.x;
            self.cpu.y = save_state.cpu_state.y;
            self.cpu.status = rnes_cpu6502::StatusFlags::from_bits(save_state.cpu_state.status).unwrap_or_default();
            self.state.cpu_cycles = save_state.cpu_state.cycles as u32;
            
            // Load PPU state
            let ppu = self.bus.ppu_mut();
            ppu.set_scanline(save_state.ppu_state.scanline as i32);
            ppu.set_dot(save_state.ppu_state.dot as u32);
            ppu.set_frame(save_state.ppu_state.frame);
            ppu.set_vblank(save_state.ppu_state.vblank);
            ppu.set_oam(save_state.ppu_state.oam);
            ppu.set_palette_ram(save_state.ppu_state.palette_ram);
                
            // Convert frame buffer back from u32
            let frame_buffer: Vec<rnes_common::Pixel> = save_state.ppu_state.frame_buffer.iter()
                .map(|&pixel| rnes_common::Pixel::from_u32(pixel))
                .collect();
            ppu.set_frame_buffer(frame_buffer);
            
            // Load memory state
            if save_state.memory_state.ram.len() == self.bus.ram.len() {
                self.bus.ram.copy_from_slice(&save_state.memory_state.ram);
            }
            let mapper = self.bus.mapper_mut();
            if !save_state.memory_state.prg_ram.is_empty() {
                mapper.load_prg_ram(&save_state.memory_state.prg_ram)?;
            }
            
            tracing::info!("Loaded save state from slot {}", slot);
        }
        Ok(())
    }
    
    /// Check if save state exists
    pub fn has_save_state(&self, slot: u8) -> bool {
        if let Some(ref rom_name) = self.rom_name {
            SaveState::exists(&self.save_system, rom_name, slot)
        } else {
            false
        }
    }
    
    /// Delete save state
    pub fn delete_save_state(&self, slot: u8) -> RnesResult<()> {
        if let Some(ref rom_name) = self.rom_name {
            SaveState::delete(&self.save_system, rom_name, slot)
        } else {
            Ok(())
        }
    }
    
    /// Quick save
    pub fn quick_save(&self) -> RnesResult<()> {
        let slot = self.config.save_states.quick_save_slot;
        self.save_state(slot)
    }
    
    /// Quick load
    pub fn quick_load(&mut self) -> RnesResult<()> {
        let slot = self.config.save_states.quick_load_slot;
        self.load_state(slot)
    }
    
    /// Get configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    /// Get mutable configuration
    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
    
    /// Save configuration
    pub fn save_config(&self) -> RnesResult<()> {
        let config_path = Config::get_config_path();
        self.config.save_to_file(config_path)
    }
    
    /// Get debugger
    pub fn get_debugger(&self) -> &Debugger {
        &self.debugger
    }
    
    /// Get mutable debugger
    pub fn get_debugger_mut(&mut self) -> &mut Debugger {
        &mut self.debugger
    }
    
    /// Add breakpoint
    pub fn add_breakpoint(&mut self, address: rnes_common::Word) {
        self.debugger.add_breakpoint(address);
    }
    
    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, address: rnes_common::Word) -> bool {
        self.debugger.remove_breakpoint(address)
    }
    
    /// Enable step mode
    pub fn enable_step_mode(&mut self) {
        self.debugger.enable_step_mode();
    }
    
    /// Disable step mode
    pub fn disable_step_mode(&mut self) {
        self.debugger.disable_step_mode();
    }
    
    /// Step to next instruction
    pub fn step_instruction(&mut self) -> RnesResult<()> {
        self.debugger.break_next_instruction();
        self.start();
        
        // Run until next instruction
        while self.is_running() {
            self.step()?;
        }
        
        Ok(())
    }
    
    /// Get memory dump
    pub fn get_memory_dump(&self, start: rnes_common::Word, length: usize) -> Vec<rnes_common::Byte> {
        let mut data = Vec::new();
        for addr in start..start + length as rnes_common::Word {
            if let Ok(byte) = self.bus.read_byte(addr) {
                data.push(byte);
            } else {
                data.push(0);
            }
        }
        data
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}
