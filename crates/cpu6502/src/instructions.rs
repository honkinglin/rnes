use crate::{Cpu, AddressingMode, StatusFlags};
use rnes_common::{Byte, Cycles, RnesResult, MemoryAccess};

/// Instruction information
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Byte,
    pub name: &'static str,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
    pub undocumented: bool,
}

/// Instruction execution function type
pub type InstructionFn = fn(&mut Cpu, AddressingMode) -> RnesResult<Cycles>;

/// Instruction table
pub static INSTRUCTIONS: [Option<Instruction>; 256] = {
    let mut table = [None; 256];
    
    // This will be filled with all instruction definitions
    // For simplicity, we'll define some basic instructions first
    
    // BRK - Break
    table[0x00] = Some(Instruction {
        opcode: 0x00,
        name: "BRK",
        addressing_mode: AddressingMode::Implied,
        cycles: 7,
        undocumented: false,
    });
    
    // ADC - Add with Carry
    table[0x69] = Some(Instruction {
        opcode: 0x69,
        name: "ADC",
        addressing_mode: AddressingMode::Immediate,
        cycles: 2,
        undocumented: false,
    });
    
    // LDA - Load Accumulator
    table[0xA9] = Some(Instruction {
        opcode: 0xA9,
        name: "LDA",
        addressing_mode: AddressingMode::Immediate,
        cycles: 2,
        undocumented: false,
    });
    
    // STA - Store Accumulator
    table[0x85] = Some(Instruction {
        opcode: 0x85,
        name: "STA",
        addressing_mode: AddressingMode::ZeroPage,
        cycles: 3,
        undocumented: false,
    });
    
    // JMP - Jump
    table[0x4C] = Some(Instruction {
        opcode: 0x4C,
        name: "JMP",
        addressing_mode: AddressingMode::Absolute,
        cycles: 3,
        undocumented: false,
    });
    
    table
};

/// Execute instruction
pub fn execute_instruction<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, opcode: Byte) -> RnesResult<Cycles> {
    let instruction = INSTRUCTIONS[opcode as usize]
        .ok_or_else(|| rnes_common::RnesError::Cpu(format!("Unknown instruction: 0x{:02X}", opcode)))?;
    
    let cycles = match opcode {
        0x00 => brk(cpu, memory, instruction.addressing_mode)?,
        0x69 => adc(cpu, memory, instruction.addressing_mode)?,
        0xA9 => lda(cpu, memory, instruction.addressing_mode)?,
        0x85 => sta(cpu, memory, instruction.addressing_mode)?,
        0x4C => jmp(cpu, memory, instruction.addressing_mode)?,
        _ => return Err(rnes_common::RnesError::Cpu(format!("Unimplemented instruction: 0x{:02X}", opcode))),
    };
    
    // Check page boundary crossing
    let extra_cycle = if instruction.addressing_mode.needs_extra_cycle(cpu, memory) { 1 } else { 0 };
    
    Ok(cycles + extra_cycle as Cycles)
}

// Instruction implementations

/// ADC - Add with Carry
fn adc<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    let old_a = cpu.a;
    let carry = cpu.status.contains(StatusFlags::CARRY) as Byte;
    
    let sum = cpu.a as u16 + operand as u16 + carry as u16;
    cpu.a = sum as Byte;
    
    // Set flags
    let overflow = ((old_a ^ operand) & 0x80 == 0) && ((old_a ^ cpu.a) & 0x80 != 0);
    cpu.status.set_czn(cpu.a, sum > 0xFF);
    cpu.status.set_overflow(overflow);
    
    Ok(2)
}

/// LDA - Load Accumulator
fn lda<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.a = operand;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}

/// STA - Store Accumulator
fn sta<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    mode.write_operand(cpu, memory, cpu.a)?;
    Ok(3)
}

/// BRK - Break
fn brk<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    // Set break flag
    cpu.status.set(StatusFlags::BREAK, true);
    
    // Save state to stack
    cpu.push_word(memory, cpu.pc + 1)?;
    cpu.push_byte(memory, cpu.status.bits())?;
    
    // Set interrupt disable flag
    cpu.status.set_interrupt_disable(true);
    
    // Jump to IRQ vector
    cpu.pc = memory.read_word(0xFFFE)?;
    
    Ok(7)
}

/// JMP - Jump
fn jmp<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    cpu.pc = addr;
    Ok(3)
}
