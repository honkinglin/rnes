use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::{Byte, Word};

/// Debugger for NES emulator
#[derive(Debug, Clone)]
pub struct Debugger {
    /// Breakpoints
    pub breakpoints: HashSet<Word>,
    /// Watchpoints (memory addresses to monitor)
    pub watchpoints: HashSet<Word>,
    /// Step mode enabled
    pub step_mode: bool,
    /// Break on next instruction
    pub break_next: bool,
    /// Debug information
    pub debug_info: DebugInfo,
    /// Memory history (for memory viewer)
    pub memory_history: Vec<MemoryAccess>,
    /// Instruction history
    pub instruction_history: Vec<InstructionInfo>,
    /// Maximum history size
    pub max_history: usize,
}

/// Debug information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Current instruction address
    pub current_pc: Word,
    /// Current instruction bytes
    pub current_instruction: Vec<Byte>,
    /// Current instruction mnemonic
    pub current_mnemonic: String,
    /// Current instruction addressing mode
    pub current_addressing_mode: String,
    /// Current instruction cycles
    pub current_cycles: u8,
    /// CPU registers
    pub cpu_registers: CpuRegisters,
    /// PPU state
    pub ppu_state: PpuDebugState,
    /// Memory access count
    pub memory_access_count: u64,
    /// Total cycles executed
    pub total_cycles: u64,
}

/// CPU registers for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuRegisters {
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,
    pub sp: Byte,
    pub pc: Word,
    pub status: Byte,
    pub status_flags: StatusFlagsDebug,
}

/// Status flags for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusFlagsDebug {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub r#break: bool,
    pub overflow: bool,
    pub negative: bool,
}

/// PPU debug state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuDebugState {
    pub scanline: i32,
    pub dot: u32,
    pub frame: u64,
    pub vblank: bool,
    pub sprite_overflow: bool,
    pub sprite_zero_hit: bool,
    pub registers: PpuRegistersDebug,
}

/// PPU registers for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuRegistersDebug {
    pub ppuctrl: Byte,
    pub ppumask: Byte,
    pub ppustatus: Byte,
    pub oamaddr: Byte,
    pub oamdata: Byte,
    pub ppuscroll: Byte,
    pub ppuaddr: Byte,
    pub ppudata: Byte,
}

/// Memory access record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccess {
    pub address: Word,
    pub value: Byte,
    pub is_write: bool,
    pub cycle: u64,
    pub pc: Word,
}

/// Instruction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionInfo {
    pub pc: Word,
    pub instruction: Vec<Byte>,
    pub mnemonic: String,
    pub addressing_mode: String,
    pub cycles: u8,
    pub cpu_state: CpuRegisters,
    pub cycle: u64,
}

impl Debugger {
    /// Create new debugger
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
            watchpoints: HashSet::new(),
            step_mode: false,
            break_next: false,
            debug_info: DebugInfo::default(),
            memory_history: Vec::new(),
            instruction_history: Vec::new(),
            max_history: 1000,
        }
    }
    
    /// Add breakpoint
    pub fn add_breakpoint(&mut self, address: Word) {
        self.breakpoints.insert(address);
        tracing::info!("Breakpoint added at 0x{:04X}", address);
    }
    
    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, address: Word) -> bool {
        let removed = self.breakpoints.remove(&address);
        if removed {
            tracing::info!("Breakpoint removed at 0x{:04X}", address);
        }
        removed
    }
    
    /// Check if breakpoint exists
    pub fn has_breakpoint(&self, address: Word) -> bool {
        self.breakpoints.contains(&address)
    }
    
    /// Add watchpoint
    pub fn add_watchpoint(&mut self, address: Word) {
        self.watchpoints.insert(address);
        tracing::info!("Watchpoint added at 0x{:04X}", address);
    }
    
    /// Remove watchpoint
    pub fn remove_watchpoint(&mut self, address: Word) -> bool {
        let removed = self.watchpoints.remove(&address);
        if removed {
            tracing::info!("Watchpoint removed at 0x{:04X}", address);
        }
        removed
    }
    
    /// Check if watchpoint exists
    pub fn has_watchpoint(&self, address: Word) -> bool {
        self.watchpoints.contains(&address)
    }
    
    /// Enable step mode
    pub fn enable_step_mode(&mut self) {
        self.step_mode = true;
        tracing::info!("Step mode enabled");
    }
    
    /// Disable step mode
    pub fn disable_step_mode(&mut self) {
        self.step_mode = false;
        tracing::info!("Step mode disabled");
    }
    
    /// Set break on next instruction
    pub fn break_next_instruction(&mut self) {
        self.break_next = true;
        tracing::info!("Break on next instruction enabled");
    }
    
    /// Check if should break
    pub fn should_break(&self, pc: Word) -> bool {
        self.breakpoints.contains(&pc) || 
        self.break_next || 
        (self.step_mode && self.debug_info.current_pc != pc && self.debug_info.current_pc != 0)
    }
    
    /// Update debug info
    pub fn update_debug_info(&mut self, info: DebugInfo) {
        let total_cycles = info.total_cycles;
        self.debug_info = info.clone();
        
        // Add to instruction history
        let instruction_info = InstructionInfo {
            pc: info.current_pc,
            instruction: info.current_instruction.clone(),
            mnemonic: info.current_mnemonic.clone(),
            addressing_mode: info.current_addressing_mode.clone(),
            cycles: info.current_cycles,
            cpu_state: info.cpu_registers.clone(),
            cycle: total_cycles,
        };
        
        self.instruction_history.push(instruction_info);
        
        // Limit history size
        if self.instruction_history.len() > self.max_history {
            self.instruction_history.remove(0);
        }
    }
    
    /// Record memory access
    pub fn record_memory_access(&mut self, access: MemoryAccess) {
        self.memory_history.push(access);
        
        // Limit history size
        if self.memory_history.len() > self.max_history {
            self.memory_history.remove(0);
        }
    }
    
    /// Get memory dump
    pub fn get_memory_dump(&self, _start: Word, length: usize) -> Vec<Byte> {
        // This would be implemented by the emulator to provide memory data
        vec![0; length]
    }
    
    /// Get disassembly
    pub fn get_disassembly(&self, _start: Word, _length: usize) -> Vec<DisassemblyLine> {
        // This would be implemented by the emulator to provide disassembly
        vec![]
    }
    
    /// Clear breakpoints
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
        tracing::info!("All breakpoints cleared");
    }
    
    /// Clear watchpoints
    pub fn clear_watchpoints(&mut self) {
        self.watchpoints.clear();
        tracing::info!("All watchpoints cleared");
    }
    
    /// Clear history
    pub fn clear_history(&mut self) {
        self.memory_history.clear();
        self.instruction_history.clear();
        tracing::info!("Debug history cleared");
    }
    
    /// Get breakpoint list
    pub fn get_breakpoints(&self) -> Vec<Word> {
        self.breakpoints.iter().cloned().collect()
    }
    
    /// Get watchpoint list
    pub fn get_watchpoints(&self) -> Vec<Word> {
        self.watchpoints.iter().cloned().collect()
    }
    
    /// Get recent memory accesses
    pub fn get_recent_memory_accesses(&self, count: usize) -> Vec<&MemoryAccess> {
        let start = if self.memory_history.len() > count {
            self.memory_history.len() - count
        } else {
            0
        };
        self.memory_history[start..].iter().collect()
    }
    
    /// Get recent instructions
    pub fn get_recent_instructions(&self, count: usize) -> Vec<&InstructionInfo> {
        let start = if self.instruction_history.len() > count {
            self.instruction_history.len() - count
        } else {
            0
        };
        self.instruction_history[start..].iter().collect()
    }
}

/// Disassembly line
#[derive(Debug, Clone)]
pub struct DisassemblyLine {
    pub address: Word,
    pub bytes: Vec<Byte>,
    pub mnemonic: String,
    pub operands: String,
    pub cycles: u8,
}

impl Default for DebugInfo {
    fn default() -> Self {
        Self {
            current_pc: 0,
            current_instruction: Vec::new(),
            current_mnemonic: String::new(),
            current_addressing_mode: String::new(),
            current_cycles: 0,
            cpu_registers: CpuRegisters::default(),
            ppu_state: PpuDebugState::default(),
            memory_access_count: 0,
            total_cycles: 0,
        }
    }
}

impl Default for CpuRegisters {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0,
            status: 0,
            status_flags: StatusFlagsDebug::default(),
        }
    }
}

impl Default for StatusFlagsDebug {
    fn default() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: true,
            decimal: false,
            r#break: false,
            overflow: false,
            negative: false,
        }
    }
}

impl Default for PpuDebugState {
    fn default() -> Self {
        Self {
            scanline: -1,
            dot: 0,
            frame: 0,
            vblank: false,
            sprite_overflow: false,
            sprite_zero_hit: false,
            registers: PpuRegistersDebug::default(),
        }
    }
}

impl Default for PpuRegistersDebug {
    fn default() -> Self {
        Self {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
        }
    }
}
