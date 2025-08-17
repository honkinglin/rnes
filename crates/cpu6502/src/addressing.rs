use crate::cpu::Cpu;
use rnes_common::{Byte, Word, RnesResult, MemoryAccess};

/// Addressing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Implied addressing
    Implied,
    /// Accumulator addressing
    Accumulator,
    /// Immediate addressing
    Immediate,
    /// Zero page addressing
    ZeroPage,
    /// Zero page X addressing
    ZeroPageX,
    /// Zero page Y addressing
    ZeroPageY,
    /// Relative addressing
    Relative,
    /// Absolute addressing
    Absolute,
    /// Absolute X addressing
    AbsoluteX,
    /// Absolute Y addressing
    AbsoluteY,
    /// Indirect addressing
    Indirect,
    /// Indirect X addressing
    IndirectX,
    /// Indirect Y addressing
    IndirectY,
}

impl AddressingMode {
    /// Get operand address
    pub fn get_address<M: MemoryAccess>(&self, cpu: &Cpu, memory: &M) -> RnesResult<Word> {
        match self {
            AddressingMode::Implied | AddressingMode::Accumulator => {
                Err(rnes_common::RnesError::Cpu("Implied addressing mode has no address".to_string()))
            }
            AddressingMode::Immediate => {
                Ok(cpu.pc)
            }
            AddressingMode::ZeroPage => {
                let addr = memory.read_byte(cpu.pc)? as Word;
                Ok(addr)
            }
            AddressingMode::ZeroPageX => {
                let addr = (memory.read_byte(cpu.pc)? as Word + cpu.x as Word) & 0xFF;
                Ok(addr)
            }
            AddressingMode::ZeroPageY => {
                let addr = (memory.read_byte(cpu.pc)? as Word + cpu.y as Word) & 0xFF;
                Ok(addr)
            }
            AddressingMode::Relative => {
                let offset = memory.read_byte(cpu.pc)? as i8 as i16;
                let addr = (cpu.pc as i16 + 1 + offset) as Word;
                Ok(addr)
            }
            AddressingMode::Absolute => {
                let addr = memory.read_word(cpu.pc)?;
                Ok(addr)
            }
            AddressingMode::AbsoluteX => {
                let addr = memory.read_word(cpu.pc)? + cpu.x as Word;
                Ok(addr)
            }
            AddressingMode::AbsoluteY => {
                let addr = memory.read_word(cpu.pc)? + cpu.y as Word;
                Ok(addr)
            }
            AddressingMode::Indirect => {
                let addr = memory.read_word(cpu.pc)?;
                let indirect_addr = memory.read_word(addr)?;
                Ok(indirect_addr)
            }
            AddressingMode::IndirectX => {
                let zp_addr = (memory.read_byte(cpu.pc)? as Word + cpu.x as Word) & 0xFF;
                let addr = memory.read_word(zp_addr)?;
                Ok(addr)
            }
            AddressingMode::IndirectY => {
                let zp_addr = memory.read_byte(cpu.pc)? as Word;
                let addr = memory.read_word(zp_addr)? + cpu.y as Word;
                Ok(addr)
            }
        }
    }
    
    /// Get operand value
    pub fn get_operand<M: MemoryAccess>(&self, cpu: &Cpu, memory: &M) -> RnesResult<Byte> {
        match self {
            AddressingMode::Implied | AddressingMode::Accumulator => {
                Err(rnes_common::RnesError::Cpu("Implied addressing mode has no operand".to_string()))
            }
            AddressingMode::Immediate => {
                memory.read_byte(cpu.pc)
            }
            _ => {
                let addr = self.get_address(cpu, memory)?;
                memory.read_byte(addr)
            }
        }
    }
    
    /// Write operand
    pub fn write_operand<M: MemoryAccess>(&self, cpu: &mut Cpu, memory: &mut M, value: Byte) -> RnesResult<()> {
        match self {
            AddressingMode::Implied | AddressingMode::Accumulator => {
                Err(rnes_common::RnesError::Cpu("Implied addressing mode cannot write".to_string()))
            }
            AddressingMode::Immediate => {
                Err(rnes_common::RnesError::Cpu("Immediate addressing mode cannot write".to_string()))
            }
            _ => {
                let addr = self.get_address(cpu, memory)?;
                memory.write_byte(addr, value)
            }
        }
    }
    
    /// Check if extra cycle is needed (page boundary crossing)
    pub fn needs_extra_cycle<M: MemoryAccess>(&self, cpu: &Cpu, memory: &M) -> bool {
        match self {
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
                let base_addr = memory.read_word(cpu.pc).unwrap_or(0);
                let final_addr = base_addr + match self {
                    AddressingMode::AbsoluteX => cpu.x as Word,
                    AddressingMode::AbsoluteY => cpu.y as Word,
                    _ => 0,
                };
                (base_addr & 0xFF00) != (final_addr & 0xFF00)
            }
            AddressingMode::IndirectY => {
                let zp_addr = memory.read_byte(cpu.pc).unwrap_or(0) as Word;
                let base_addr = memory.read_word(zp_addr).unwrap_or(0);
                let final_addr = base_addr + cpu.y as Word;
                (base_addr & 0xFF00) != (final_addr & 0xFF00)
            }
            _ => false,
        }
    }
}
