use serde::{Deserialize, Serialize};
use rnes_common::{Byte, Word, Cycles, RnesResult, MemoryAccess};
use crate::{StatusFlags, execute_instruction};

/// 6502 CPU implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    // Registers
    pub a: Byte,      // Accumulator
    pub x: Byte,      // X index register
    pub y: Byte,      // Y index register
    pub sp: Byte,     // Stack pointer
    pub pc: Word,     // Program counter
    pub status: StatusFlags, // Status register
    
    // Internal state
    pub cycles: Cycles, // Total cycles
    pub stall_cycles: Cycles, // Stall cycles
    
    // Interrupt state
    pub nmi_pending: bool,
    pub irq_pending: bool,
    pub reset_pending: bool,
}

impl Cpu {
    /// Create new CPU instance
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD, // Stack pointer initial value
            pc: 0,
            status: StatusFlags::default(),
            cycles: 0,
            stall_cycles: 0,
            nmi_pending: false,
            irq_pending: false,
            reset_pending: false,
        }
    }
    
    /// Reset CPU
    pub fn reset<M: MemoryAccess>(&mut self, memory: &mut M) -> RnesResult<()> {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.status = StatusFlags::default();
        self.cycles = 0;
        self.stall_cycles = 0;
        self.nmi_pending = false;
        self.irq_pending = false;
        self.reset_pending = false;
        
        // Read reset vector
        self.pc = memory.read_word(0xFFFC)?;
        
        Ok(())
    }
    
    /// Execute one CPU cycle
    pub fn step<M: MemoryAccess>(&mut self, memory: &mut M) -> RnesResult<Cycles> {
        if self.stall_cycles > 0 {
            self.stall_cycles -= 1;
            self.cycles += 1;
            return Ok(1);
        }
        
        // Handle interrupts
        if self.nmi_pending {
            self.handle_nmi(memory)?;
        } else if self.irq_pending && !self.status.interrupts_disabled() {
            self.handle_irq(memory)?;
        }
        
        // Fetch and execute instruction
        let opcode = memory.read_byte(self.pc)?;
        let instruction_cycles = execute_instruction(self, memory, opcode)?;
        
        // Update program counter (except for jump instructions)
        if opcode != 0x4C { // JMP
            self.pc += 1;
        }
        
        self.cycles += instruction_cycles;
        Ok(instruction_cycles)
    }
    
    /// Handle NMI interrupt
    fn handle_nmi<M: MemoryAccess>(&mut self, memory: &mut M) -> RnesResult<()> {
        self.nmi_pending = false;
        
        // Save state to stack
        self.push_word(memory, self.pc)?;
        self.push_byte(memory, self.status.bits())?;
        
        // Set interrupt disable flag
        self.status.set_interrupt_disable(true);
        
        // Jump to NMI vector
        self.pc = memory.read_word(0xFFFA)?;
        
        Ok(())
    }
    
    /// Handle IRQ interrupt
    fn handle_irq<M: MemoryAccess>(&mut self, memory: &mut M) -> RnesResult<()> {
        self.irq_pending = false;
        
        // Save state to stack
        self.push_word(memory, self.pc)?;
        self.push_byte(memory, self.status.bits())?;
        
        // Set interrupt disable flag
        self.status.set_interrupt_disable(true);
        
        // Jump to IRQ vector
        self.pc = memory.read_word(0xFFFE)?;
        
        Ok(())
    }
    
    /// Request NMI interrupt
    pub fn request_nmi(&mut self) {
        self.nmi_pending = true;
    }
    
    /// Request IRQ interrupt
    pub fn request_irq(&mut self) {
        self.irq_pending = true;
    }
    
    /// Stall CPU for specified cycles
    pub fn stall(&mut self, cycles: Cycles) {
        self.stall_cycles += cycles;
    }
    
    // Stack operations
    
    /// Push byte to stack
    pub fn push_byte<M: MemoryAccess>(&mut self, memory: &mut M, value: Byte) -> RnesResult<()> {
        memory.write_byte(0x0100 + self.sp as Word, value)?;
        self.sp = self.sp.wrapping_sub(1);
        Ok(())
    }
    
    /// Pop byte from stack
    pub fn pop_byte<M: MemoryAccess>(&mut self, memory: &M) -> RnesResult<Byte> {
        self.sp = self.sp.wrapping_add(1);
        memory.read_byte(0x0100 + self.sp as Word)
    }
    
    /// Push word to stack
    pub fn push_word<M: MemoryAccess>(&mut self, memory: &mut M, value: Word) -> RnesResult<()> {
        self.push_byte(memory, (value >> 8) as Byte)?;
        self.push_byte(memory, value as Byte)?;
        Ok(())
    }
    
    /// Pop word from stack
    pub fn pop_word<M: MemoryAccess>(&mut self, memory: &M) -> RnesResult<Word> {
        let low = self.pop_byte(memory)? as Word;
        let high = self.pop_byte(memory)? as Word;
        Ok(low | (high << 8))
    }
    
    /// Get CPU status string
    pub fn status_string(&self) -> String {
        format!(
            "A:{:02X} X:{:02X} Y:{:02X} SP:{:02X} PC:{:04X} Status:{:02X}",
            self.a, self.x, self.y, self.sp, self.pc, self.status.bits()
        )
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
