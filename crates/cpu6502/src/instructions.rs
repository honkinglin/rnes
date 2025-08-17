use crate::{Cpu, AddressingMode, StatusFlags};
use rnes_common::{Byte, Word, Cycles, RnesResult, MemoryAccess};

/// Get instruction length based on addressing mode
pub fn get_instruction_length(opcode: Byte) -> u16 {
    if let Some(instruction) = INSTRUCTIONS[opcode as usize] {
        match instruction.addressing_mode {
            AddressingMode::Implied | AddressingMode::Accumulator => 1,
            AddressingMode::Immediate | AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY | AddressingMode::Relative | AddressingMode::IndirectX | AddressingMode::IndirectY => 2,
            AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY | AddressingMode::Indirect => 3,
        }
    } else {
        1 // Default to 1 byte for unknown instructions
    }
}

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

/// Instruction table - Complete 6502 instruction set
pub static INSTRUCTIONS: [Option<Instruction>; 256] = {
    let mut table = [None; 256];
    
    // ADC - Add with Carry
    table[0x69] = Some(Instruction { opcode: 0x69, name: "ADC", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0x65] = Some(Instruction { opcode: 0x65, name: "ADC", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x75] = Some(Instruction { opcode: 0x75, name: "ADC", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0x6D] = Some(Instruction { opcode: 0x6D, name: "ADC", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0x7D] = Some(Instruction { opcode: 0x7D, name: "ADC", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0x79] = Some(Instruction { opcode: 0x79, name: "ADC", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0x61] = Some(Instruction { opcode: 0x61, name: "ADC", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0x71] = Some(Instruction { opcode: 0x71, name: "ADC", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // AND - Logical AND
    table[0x29] = Some(Instruction { opcode: 0x29, name: "AND", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0x25] = Some(Instruction { opcode: 0x25, name: "AND", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x35] = Some(Instruction { opcode: 0x35, name: "AND", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0x2D] = Some(Instruction { opcode: 0x2D, name: "AND", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0x3D] = Some(Instruction { opcode: 0x3D, name: "AND", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0x39] = Some(Instruction { opcode: 0x39, name: "AND", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0x21] = Some(Instruction { opcode: 0x21, name: "AND", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0x31] = Some(Instruction { opcode: 0x31, name: "AND", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // ASL - Arithmetic Shift Left
    table[0x0A] = Some(Instruction { opcode: 0x0A, name: "ASL", addressing_mode: AddressingMode::Accumulator, cycles: 2, undocumented: false });
    table[0x06] = Some(Instruction { opcode: 0x06, name: "ASL", addressing_mode: AddressingMode::ZeroPage, cycles: 5, undocumented: false });
    table[0x16] = Some(Instruction { opcode: 0x16, name: "ASL", addressing_mode: AddressingMode::ZeroPageX, cycles: 6, undocumented: false });
    table[0x0E] = Some(Instruction { opcode: 0x0E, name: "ASL", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    table[0x1E] = Some(Instruction { opcode: 0x1E, name: "ASL", addressing_mode: AddressingMode::AbsoluteX, cycles: 7, undocumented: false });
    
    // BCC - Branch if Carry Clear
    table[0x90] = Some(Instruction { opcode: 0x90, name: "BCC", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BCS - Branch if Carry Set
    table[0xB0] = Some(Instruction { opcode: 0xB0, name: "BCS", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BEQ - Branch if Equal
    table[0xF0] = Some(Instruction { opcode: 0xF0, name: "BEQ", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BIT - Bit Test
    table[0x24] = Some(Instruction { opcode: 0x24, name: "BIT", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x2C] = Some(Instruction { opcode: 0x2C, name: "BIT", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    
    // BMI - Branch if Minus
    table[0x30] = Some(Instruction { opcode: 0x30, name: "BMI", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BNE - Branch if Not Equal
    table[0xD0] = Some(Instruction { opcode: 0xD0, name: "BNE", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BPL - Branch if Positive
    table[0x10] = Some(Instruction { opcode: 0x10, name: "BPL", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BRK - Break
    table[0x00] = Some(Instruction { opcode: 0x00, name: "BRK", addressing_mode: AddressingMode::Implied, cycles: 7, undocumented: false });
    
    // BVC - Branch if Overflow Clear
    table[0x50] = Some(Instruction { opcode: 0x50, name: "BVC", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // BVS - Branch if Overflow Set
    table[0x70] = Some(Instruction { opcode: 0x70, name: "BVS", addressing_mode: AddressingMode::Relative, cycles: 2, undocumented: false });
    
    // CLC - Clear Carry Flag
    table[0x18] = Some(Instruction { opcode: 0x18, name: "CLC", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // CLD - Clear Decimal Flag
    table[0xD8] = Some(Instruction { opcode: 0xD8, name: "CLD", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // CLI - Clear Interrupt Disable
    table[0x58] = Some(Instruction { opcode: 0x58, name: "CLI", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // CLV - Clear Overflow Flag
    table[0xB8] = Some(Instruction { opcode: 0xB8, name: "CLV", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // CMP - Compare
    table[0xC9] = Some(Instruction { opcode: 0xC9, name: "CMP", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xC5] = Some(Instruction { opcode: 0xC5, name: "CMP", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xD5] = Some(Instruction { opcode: 0xD5, name: "CMP", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0xCD] = Some(Instruction { opcode: 0xCD, name: "CMP", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0xDD] = Some(Instruction { opcode: 0xDD, name: "CMP", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0xD9] = Some(Instruction { opcode: 0xD9, name: "CMP", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0xC1] = Some(Instruction { opcode: 0xC1, name: "CMP", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0xD1] = Some(Instruction { opcode: 0xD1, name: "CMP", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // CPX - Compare X Register
    table[0xE0] = Some(Instruction { opcode: 0xE0, name: "CPX", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xE4] = Some(Instruction { opcode: 0xE4, name: "CPX", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xEC] = Some(Instruction { opcode: 0xEC, name: "CPX", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    
    // CPY - Compare Y Register
    table[0xC0] = Some(Instruction { opcode: 0xC0, name: "CPY", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xC4] = Some(Instruction { opcode: 0xC4, name: "CPY", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xCC] = Some(Instruction { opcode: 0xCC, name: "CPY", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    
    // DEC - Decrement Memory
    table[0xC6] = Some(Instruction { opcode: 0xC6, name: "DEC", addressing_mode: AddressingMode::ZeroPage, cycles: 5, undocumented: false });
    table[0xD6] = Some(Instruction { opcode: 0xD6, name: "DEC", addressing_mode: AddressingMode::ZeroPageX, cycles: 6, undocumented: false });
    table[0xCE] = Some(Instruction { opcode: 0xCE, name: "DEC", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    table[0xDE] = Some(Instruction { opcode: 0xDE, name: "DEC", addressing_mode: AddressingMode::AbsoluteX, cycles: 7, undocumented: false });
    
    // DEX - Decrement X Register
    table[0xCA] = Some(Instruction { opcode: 0xCA, name: "DEX", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // DEY - Decrement Y Register
    table[0x88] = Some(Instruction { opcode: 0x88, name: "DEY", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // EOR - Exclusive OR
    table[0x49] = Some(Instruction { opcode: 0x49, name: "EOR", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0x45] = Some(Instruction { opcode: 0x45, name: "EOR", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x55] = Some(Instruction { opcode: 0x55, name: "EOR", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0x4D] = Some(Instruction { opcode: 0x4D, name: "EOR", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0x5D] = Some(Instruction { opcode: 0x5D, name: "EOR", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0x59] = Some(Instruction { opcode: 0x59, name: "EOR", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0x41] = Some(Instruction { opcode: 0x41, name: "EOR", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0x51] = Some(Instruction { opcode: 0x51, name: "EOR", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // INC - Increment Memory
    table[0xE6] = Some(Instruction { opcode: 0xE6, name: "INC", addressing_mode: AddressingMode::ZeroPage, cycles: 5, undocumented: false });
    table[0xF6] = Some(Instruction { opcode: 0xF6, name: "INC", addressing_mode: AddressingMode::ZeroPageX, cycles: 6, undocumented: false });
    table[0xEE] = Some(Instruction { opcode: 0xEE, name: "INC", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    table[0xFE] = Some(Instruction { opcode: 0xFE, name: "INC", addressing_mode: AddressingMode::AbsoluteX, cycles: 7, undocumented: false });
    
    // INX - Increment X Register
    table[0xE8] = Some(Instruction { opcode: 0xE8, name: "INX", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // INY - Increment Y Register
    table[0xC8] = Some(Instruction { opcode: 0xC8, name: "INY", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // JMP - Jump
    table[0x4C] = Some(Instruction { opcode: 0x4C, name: "JMP", addressing_mode: AddressingMode::Absolute, cycles: 3, undocumented: false });
    table[0x6C] = Some(Instruction { opcode: 0x6C, name: "JMP", addressing_mode: AddressingMode::Indirect, cycles: 5, undocumented: false });
    
    // JSR - Jump to Subroutine
    table[0x20] = Some(Instruction { opcode: 0x20, name: "JSR", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    
    // LDA - Load Accumulator
    table[0xA9] = Some(Instruction { opcode: 0xA9, name: "LDA", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xA5] = Some(Instruction { opcode: 0xA5, name: "LDA", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xB5] = Some(Instruction { opcode: 0xB5, name: "LDA", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0xAD] = Some(Instruction { opcode: 0xAD, name: "LDA", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0xBD] = Some(Instruction { opcode: 0xBD, name: "LDA", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0xB9] = Some(Instruction { opcode: 0xB9, name: "LDA", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0xA1] = Some(Instruction { opcode: 0xA1, name: "LDA", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0xB1] = Some(Instruction { opcode: 0xB1, name: "LDA", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // LDX - Load X Register
    table[0xA2] = Some(Instruction { opcode: 0xA2, name: "LDX", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xA6] = Some(Instruction { opcode: 0xA6, name: "LDX", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xB6] = Some(Instruction { opcode: 0xB6, name: "LDX", addressing_mode: AddressingMode::ZeroPageY, cycles: 4, undocumented: false });
    table[0xAE] = Some(Instruction { opcode: 0xAE, name: "LDX", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0xBE] = Some(Instruction { opcode: 0xBE, name: "LDX", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    
    // LDY - Load Y Register
    table[0xA0] = Some(Instruction { opcode: 0xA0, name: "LDY", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xA4] = Some(Instruction { opcode: 0xA4, name: "LDY", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xB4] = Some(Instruction { opcode: 0xB4, name: "LDY", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0xAC] = Some(Instruction { opcode: 0xAC, name: "LDY", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0xBC] = Some(Instruction { opcode: 0xBC, name: "LDY", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    
    // LSR - Logical Shift Right
    table[0x4A] = Some(Instruction { opcode: 0x4A, name: "LSR", addressing_mode: AddressingMode::Accumulator, cycles: 2, undocumented: false });
    table[0x46] = Some(Instruction { opcode: 0x46, name: "LSR", addressing_mode: AddressingMode::ZeroPage, cycles: 5, undocumented: false });
    table[0x56] = Some(Instruction { opcode: 0x56, name: "LSR", addressing_mode: AddressingMode::ZeroPageX, cycles: 6, undocumented: false });
    table[0x4E] = Some(Instruction { opcode: 0x4E, name: "LSR", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    table[0x5E] = Some(Instruction { opcode: 0x5E, name: "LSR", addressing_mode: AddressingMode::AbsoluteX, cycles: 7, undocumented: false });
    
    // NOP - No Operation
    table[0xEA] = Some(Instruction { opcode: 0xEA, name: "NOP", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    table[0xFF] = Some(Instruction { opcode: 0xFF, name: "NOP", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: true });
    
    // ORA - Logical Inclusive OR
    table[0x09] = Some(Instruction { opcode: 0x09, name: "ORA", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0x05] = Some(Instruction { opcode: 0x05, name: "ORA", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x15] = Some(Instruction { opcode: 0x15, name: "ORA", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0x0D] = Some(Instruction { opcode: 0x0D, name: "ORA", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0x1D] = Some(Instruction { opcode: 0x1D, name: "ORA", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0x19] = Some(Instruction { opcode: 0x19, name: "ORA", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0x01] = Some(Instruction { opcode: 0x01, name: "ORA", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0x11] = Some(Instruction { opcode: 0x11, name: "ORA", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // PHA - Push Accumulator
    table[0x48] = Some(Instruction { opcode: 0x48, name: "PHA", addressing_mode: AddressingMode::Implied, cycles: 3, undocumented: false });
    
    // PHP - Push Processor Status
    table[0x08] = Some(Instruction { opcode: 0x08, name: "PHP", addressing_mode: AddressingMode::Implied, cycles: 3, undocumented: false });
    
    // PLA - Pull Accumulator
    table[0x68] = Some(Instruction { opcode: 0x68, name: "PLA", addressing_mode: AddressingMode::Implied, cycles: 4, undocumented: false });
    
    // PLP - Pull Processor Status
    table[0x28] = Some(Instruction { opcode: 0x28, name: "PLP", addressing_mode: AddressingMode::Implied, cycles: 4, undocumented: false });
    
    // ROL - Rotate Left
    table[0x2A] = Some(Instruction { opcode: 0x2A, name: "ROL", addressing_mode: AddressingMode::Accumulator, cycles: 2, undocumented: false });
    table[0x26] = Some(Instruction { opcode: 0x26, name: "ROL", addressing_mode: AddressingMode::ZeroPage, cycles: 5, undocumented: false });
    table[0x36] = Some(Instruction { opcode: 0x36, name: "ROL", addressing_mode: AddressingMode::ZeroPageX, cycles: 6, undocumented: false });
    table[0x2E] = Some(Instruction { opcode: 0x2E, name: "ROL", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    table[0x3E] = Some(Instruction { opcode: 0x3E, name: "ROL", addressing_mode: AddressingMode::AbsoluteX, cycles: 7, undocumented: false });
    
    // ROR - Rotate Right
    table[0x6A] = Some(Instruction { opcode: 0x6A, name: "ROR", addressing_mode: AddressingMode::Accumulator, cycles: 2, undocumented: false });
    table[0x66] = Some(Instruction { opcode: 0x66, name: "ROR", addressing_mode: AddressingMode::ZeroPage, cycles: 5, undocumented: false });
    table[0x76] = Some(Instruction { opcode: 0x76, name: "ROR", addressing_mode: AddressingMode::ZeroPageX, cycles: 6, undocumented: false });
    table[0x6E] = Some(Instruction { opcode: 0x6E, name: "ROR", addressing_mode: AddressingMode::Absolute, cycles: 6, undocumented: false });
    table[0x7E] = Some(Instruction { opcode: 0x7E, name: "ROR", addressing_mode: AddressingMode::AbsoluteX, cycles: 7, undocumented: false });
    
    // RTI - Return from Interrupt
    table[0x40] = Some(Instruction { opcode: 0x40, name: "RTI", addressing_mode: AddressingMode::Implied, cycles: 6, undocumented: false });
    
    // RTS - Return from Subroutine
    table[0x60] = Some(Instruction { opcode: 0x60, name: "RTS", addressing_mode: AddressingMode::Implied, cycles: 6, undocumented: false });
    
    // SBC - Subtract with Carry
    table[0xE9] = Some(Instruction { opcode: 0xE9, name: "SBC", addressing_mode: AddressingMode::Immediate, cycles: 2, undocumented: false });
    table[0xE5] = Some(Instruction { opcode: 0xE5, name: "SBC", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0xF5] = Some(Instruction { opcode: 0xF5, name: "SBC", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0xED] = Some(Instruction { opcode: 0xED, name: "SBC", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0xFD] = Some(Instruction { opcode: 0xFD, name: "SBC", addressing_mode: AddressingMode::AbsoluteX, cycles: 4, undocumented: false });
    table[0xF9] = Some(Instruction { opcode: 0xF9, name: "SBC", addressing_mode: AddressingMode::AbsoluteY, cycles: 4, undocumented: false });
    table[0xE1] = Some(Instruction { opcode: 0xE1, name: "SBC", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0xF1] = Some(Instruction { opcode: 0xF1, name: "SBC", addressing_mode: AddressingMode::IndirectY, cycles: 5, undocumented: false });
    
    // SEC - Set Carry Flag
    table[0x38] = Some(Instruction { opcode: 0x38, name: "SEC", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // SED - Set Decimal Flag
    table[0xF8] = Some(Instruction { opcode: 0xF8, name: "SED", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // SEI - Set Interrupt Disable
    table[0x78] = Some(Instruction { opcode: 0x78, name: "SEI", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // STA - Store Accumulator
    table[0x85] = Some(Instruction { opcode: 0x85, name: "STA", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x95] = Some(Instruction { opcode: 0x95, name: "STA", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0x8D] = Some(Instruction { opcode: 0x8D, name: "STA", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    table[0x9D] = Some(Instruction { opcode: 0x9D, name: "STA", addressing_mode: AddressingMode::AbsoluteX, cycles: 5, undocumented: false });
    table[0x99] = Some(Instruction { opcode: 0x99, name: "STA", addressing_mode: AddressingMode::AbsoluteY, cycles: 5, undocumented: false });
    table[0x81] = Some(Instruction { opcode: 0x81, name: "STA", addressing_mode: AddressingMode::IndirectX, cycles: 6, undocumented: false });
    table[0x91] = Some(Instruction { opcode: 0x91, name: "STA", addressing_mode: AddressingMode::IndirectY, cycles: 6, undocumented: false });
    
    // STX - Store X Register
    table[0x86] = Some(Instruction { opcode: 0x86, name: "STX", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x96] = Some(Instruction { opcode: 0x96, name: "STX", addressing_mode: AddressingMode::ZeroPageY, cycles: 4, undocumented: false });
    table[0x8E] = Some(Instruction { opcode: 0x8E, name: "STX", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    
    // STY - Store Y Register
    table[0x84] = Some(Instruction { opcode: 0x84, name: "STY", addressing_mode: AddressingMode::ZeroPage, cycles: 3, undocumented: false });
    table[0x94] = Some(Instruction { opcode: 0x94, name: "STY", addressing_mode: AddressingMode::ZeroPageX, cycles: 4, undocumented: false });
    table[0x8C] = Some(Instruction { opcode: 0x8C, name: "STY", addressing_mode: AddressingMode::Absolute, cycles: 4, undocumented: false });
    
    // TAX - Transfer Accumulator to X
    table[0xAA] = Some(Instruction { opcode: 0xAA, name: "TAX", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // TAY - Transfer Accumulator to Y
    table[0xA8] = Some(Instruction { opcode: 0xA8, name: "TAY", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // TSX - Transfer Stack Pointer to X
    table[0xBA] = Some(Instruction { opcode: 0xBA, name: "TSX", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // TXA - Transfer X to Accumulator
    table[0x8A] = Some(Instruction { opcode: 0x8A, name: "TXA", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // TXS - Transfer X to Stack Pointer
    table[0x9A] = Some(Instruction { opcode: 0x9A, name: "TXS", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    // TYA - Transfer Y to Accumulator
    table[0x98] = Some(Instruction { opcode: 0x98, name: "TYA", addressing_mode: AddressingMode::Implied, cycles: 2, undocumented: false });
    
    table
};

/// Execute instruction
pub fn execute_instruction<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, opcode: Byte) -> RnesResult<Cycles> {
    let instruction = INSTRUCTIONS[opcode as usize]
        .ok_or_else(|| rnes_common::RnesError::Cpu(format!("Unknown instruction: 0x{:02X}", opcode)))?;
    
    let cycles = match opcode {
        // ADC - Add with Carry
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => adc(cpu, memory, instruction.addressing_mode)?,
        
        // AND - Logical AND
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => and(cpu, memory, instruction.addressing_mode)?,
        
        // ASL - Arithmetic Shift Left
        0x0A => asl_acc(cpu)?,
        0x06 | 0x16 | 0x0E | 0x1E => asl_mem(cpu, memory, instruction.addressing_mode)?,
        
        // Branch instructions
        0x90 => bcc(cpu, memory, instruction.addressing_mode)?,
        0xB0 => bcs(cpu, memory, instruction.addressing_mode)?,
        0xF0 => beq(cpu, memory, instruction.addressing_mode)?,
        0x30 => bmi(cpu, memory, instruction.addressing_mode)?,
        0xD0 => bne(cpu, memory, instruction.addressing_mode)?,
        0x10 => bpl(cpu, memory, instruction.addressing_mode)?,
        0x50 => bvc(cpu, memory, instruction.addressing_mode)?,
        0x70 => bvs(cpu, memory, instruction.addressing_mode)?,
        
        // BIT - Bit Test
        0x24 | 0x2C => bit(cpu, memory, instruction.addressing_mode)?,
        
        // BRK - Break
        0x00 => brk(cpu, memory, instruction.addressing_mode)?,
        
        // Clear flag instructions
        0x18 => clc(cpu)?,
        0xD8 => cld(cpu)?,
        0x58 => cli(cpu)?,
        0xB8 => clv(cpu)?,
        
        // Compare instructions
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => cmp(cpu, memory, instruction.addressing_mode)?,
        0xE0 | 0xE4 | 0xEC => cpx(cpu, memory, instruction.addressing_mode)?,
        0xC0 | 0xC4 | 0xCC => cpy(cpu, memory, instruction.addressing_mode)?,
        
        // Decrement instructions
        0xC6 | 0xD6 | 0xCE | 0xDE => dec(cpu, memory, instruction.addressing_mode)?,
        0xCA => dex(cpu)?,
        0x88 => dey(cpu)?,
        
        // EOR - Exclusive OR
        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => eor(cpu, memory, instruction.addressing_mode)?,
        
        // Increment instructions
        0xE6 | 0xF6 | 0xEE | 0xFE => inc(cpu, memory, instruction.addressing_mode)?,
        0xE8 => inx(cpu)?,
        0xC8 => iny(cpu)?,
        
        // JMP - Jump
        0x4C | 0x6C => jmp(cpu, memory, instruction.addressing_mode)?,
        
        // JSR - Jump to Subroutine
        0x20 => jsr(cpu, memory, instruction.addressing_mode)?,
        
        // Load instructions
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => lda(cpu, memory, instruction.addressing_mode)?,
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => ldx(cpu, memory, instruction.addressing_mode)?,
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => ldy(cpu, memory, instruction.addressing_mode)?,
        
        // LSR - Logical Shift Right
        0x4A => lsr_acc(cpu)?,
        0x46 | 0x56 | 0x4E | 0x5E => lsr_mem(cpu, memory, instruction.addressing_mode)?,
        
        // NOP - No Operation
        0xEA | 0xFF => nop()?,
        
        // ORA - Logical Inclusive OR
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => ora(cpu, memory, instruction.addressing_mode)?,
        
        // Push/Pull instructions
        0x48 => pha(cpu, memory)?,
        0x08 => php(cpu, memory)?,
        0x68 => pla(cpu, memory)?,
        0x28 => plp(cpu, memory)?,
        
        // ROL - Rotate Left
        0x2A => rol_acc(cpu)?,
        0x26 | 0x36 | 0x2E | 0x3E => rol_mem(cpu, memory, instruction.addressing_mode)?,
        
        // ROR - Rotate Right
        0x6A => ror_acc(cpu)?,
        0x66 | 0x76 | 0x6E | 0x7E => ror_mem(cpu, memory, instruction.addressing_mode)?,
        
        // Return instructions
        0x40 => rti(cpu, memory)?,
        0x60 => rts(cpu, memory)?,
        
        // SBC - Subtract with Carry
        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => sbc(cpu, memory, instruction.addressing_mode)?,
        
        // Set flag instructions
        0x38 => sec(cpu)?,
        0xF8 => sed(cpu)?,
        0x78 => sei(cpu)?,
        
        // Store instructions
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => sta(cpu, memory, instruction.addressing_mode)?,
        0x86 | 0x96 | 0x8E => stx(cpu, memory, instruction.addressing_mode)?,
        0x84 | 0x94 | 0x8C => sty(cpu, memory, instruction.addressing_mode)?,
        
        // Transfer instructions
        0xAA => tax(cpu)?,
        0xA8 => tay(cpu)?,
        0xBA => tsx(cpu)?,
        0x8A => txa(cpu)?,
        0x9A => txs(cpu)?,
        0x98 => tya(cpu)?,
        
        _ => return Err(rnes_common::RnesError::Cpu(format!("Unimplemented instruction: 0x{:02X}", opcode))),
    };
    
    // Check page boundary crossing for branch instructions
    let extra_cycle = if matches!(opcode, 0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70) {
        if instruction.addressing_mode.needs_extra_cycle(cpu, memory) { 1 } else { 0 }
    } else {
        if instruction.addressing_mode.needs_extra_cycle(cpu, memory) { 1 } else { 0 }
    };
    
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

/// AND - Logical AND
fn and<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.a &= operand;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}

/// ASL - Arithmetic Shift Left (Accumulator)
fn asl_acc(cpu: &mut Cpu) -> RnesResult<Cycles> {
    let carry = (cpu.a & 0x80) != 0;
    cpu.a <<= 1;
    cpu.status.set_czn(cpu.a, carry);
    Ok(2)
}

/// ASL - Arithmetic Shift Left (Memory)
fn asl_mem<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    let value = memory.read_byte(addr)?;
    let carry = (value & 0x80) != 0;
    let result = value << 1;
    memory.write_byte(addr, result)?;
    cpu.status.set_czn(result, carry);
    Ok(5)
}

/// BCC - Branch if Carry Clear
fn bcc<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if !cpu.status.contains(StatusFlags::CARRY) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BCS - Branch if Carry Set
fn bcs<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if cpu.status.contains(StatusFlags::CARRY) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BEQ - Branch if Equal
fn beq<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if cpu.status.contains(StatusFlags::ZERO) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BIT - Bit Test
fn bit<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    let result = cpu.a & operand;
    
    cpu.status.set(StatusFlags::ZERO, result == 0);
    cpu.status.set(StatusFlags::NEGATIVE, (operand & 0x80) != 0);
    cpu.status.set(StatusFlags::OVERFLOW, (operand & 0x40) != 0);
    
    Ok(3)
}

/// BMI - Branch if Minus
fn bmi<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if cpu.status.contains(StatusFlags::NEGATIVE) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BNE - Branch if Not Equal
fn bne<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if !cpu.status.contains(StatusFlags::ZERO) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BPL - Branch if Positive
fn bpl<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if !cpu.status.contains(StatusFlags::NEGATIVE) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BVC - Branch if Overflow Clear
fn bvc<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if !cpu.status.contains(StatusFlags::OVERFLOW) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
}

/// BVS - Branch if Overflow Set
fn bvs<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, _mode: AddressingMode) -> RnesResult<Cycles> {
    if cpu.status.contains(StatusFlags::OVERFLOW) {
        let offset = memory.read_byte(cpu.pc + 1)? as i8 as i16;
        cpu.pc = (cpu.pc as i16 + 2 + offset) as Word;
        Ok(3)
    } else {
        Ok(2)
    }
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
    let irq_vector = memory.read_word(0xFFFE)?;
    
    // If IRQ vector points to invalid address (like 0x0000), 
    // start from PRG ROM base address (0x8000)
    if irq_vector < 0x8000 {
        cpu.pc = 0x8000;
    } else {
        cpu.pc = irq_vector;
    }
    
    Ok(7)
}

/// CLC - Clear Carry Flag
fn clc(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.remove(StatusFlags::CARRY);
    Ok(2)
}

/// CLD - Clear Decimal Flag
fn cld(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.remove(StatusFlags::DECIMAL);
    Ok(2)
}

/// CLI - Clear Interrupt Disable
fn cli(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.remove(StatusFlags::INTERRUPT_DISABLE);
    Ok(2)
}

/// CLV - Clear Overflow Flag
fn clv(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.remove(StatusFlags::OVERFLOW);
    Ok(2)
}

/// CMP - Compare
fn cmp<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    let result = cpu.a.wrapping_sub(operand);
    cpu.status.set_czn(result, cpu.a >= operand);
    Ok(2)
}

/// CPX - Compare X Register
fn cpx<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    let result = cpu.x.wrapping_sub(operand);
    cpu.status.set_czn(result, cpu.x >= operand);
    Ok(2)
}

/// CPY - Compare Y Register
fn cpy<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    let result = cpu.y.wrapping_sub(operand);
    cpu.status.set_czn(result, cpu.y >= operand);
    Ok(2)
}

/// DEC - Decrement Memory
fn dec<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    let value = memory.read_byte(addr)?;
    let result = value.wrapping_sub(1);
    memory.write_byte(addr, result)?;
    cpu.status.set_zn(result);
    Ok(5)
}

/// DEX - Decrement X Register
fn dex(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.status.set_zn(cpu.x);
    Ok(2)
}

/// DEY - Decrement Y Register
fn dey(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.y = cpu.y.wrapping_sub(1);
    cpu.status.set_zn(cpu.y);
    Ok(2)
}

/// EOR - Exclusive OR
fn eor<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.a ^= operand;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}

/// INC - Increment Memory
fn inc<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    let value = memory.read_byte(addr)?;
    let result = value.wrapping_add(1);
    memory.write_byte(addr, result)?;
    cpu.status.set_zn(result);
    Ok(5)
}

/// INX - Increment X Register
fn inx(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.status.set_zn(cpu.x);
    Ok(2)
}

/// INY - Increment Y Register
fn iny(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.status.set_zn(cpu.y);
    Ok(2)
}

/// JMP - Jump
fn jmp<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    cpu.pc = addr;
    Ok(3)
}

/// JSR - Jump to Subroutine
fn jsr<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    
    // Push return address to stack (PC points to next instruction)
    cpu.push_word(memory, cpu.pc + 1)?;
    
    // Jump to subroutine
    cpu.pc = addr;
    Ok(6)
}

/// LDA - Load Accumulator
fn lda<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.a = operand;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}

/// LDX - Load X Register
fn ldx<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.x = operand;
    cpu.status.set_zn(cpu.x);
    Ok(2)
}

/// LDY - Load Y Register
fn ldy<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.y = operand;
    cpu.status.set_zn(cpu.y);
    Ok(2)
}

/// LSR - Logical Shift Right (Accumulator)
fn lsr_acc(cpu: &mut Cpu) -> RnesResult<Cycles> {
    let carry = (cpu.a & 0x01) != 0;
    cpu.a >>= 1;
    cpu.status.set_czn(cpu.a, carry);
    Ok(2)
}

/// LSR - Logical Shift Right (Memory)
fn lsr_mem<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    let value = memory.read_byte(addr)?;
    let carry = (value & 0x01) != 0;
    let result = value >> 1;
    memory.write_byte(addr, result)?;
    cpu.status.set_czn(result, carry);
    Ok(5)
}

/// NOP - No Operation
fn nop() -> RnesResult<Cycles> {
    Ok(2)
}

/// ORA - Logical Inclusive OR
fn ora<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    cpu.a |= operand;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}

/// PHA - Push Accumulator
fn pha<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M) -> RnesResult<Cycles> {
    cpu.push_byte(memory, cpu.a)?;
    Ok(3)
}

/// PHP - Push Processor Status
fn php<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M) -> RnesResult<Cycles> {
    // Set break and unused flags when pushing
    let status = cpu.status.bits() | StatusFlags::BREAK.bits() | StatusFlags::UNUSED.bits();
    cpu.push_byte(memory, status)?;
    Ok(3)
}

/// PLA - Pull Accumulator
fn pla<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M) -> RnesResult<Cycles> {
    cpu.a = cpu.pop_byte(memory)?;
    cpu.status.set_zn(cpu.a);
    Ok(4)
}

/// PLP - Pull Processor Status
fn plp<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M) -> RnesResult<Cycles> {
    let status = cpu.pop_byte(memory)?;
    cpu.status = StatusFlags::from_bits(status).unwrap_or_default();
    Ok(4)
}

/// ROL - Rotate Left (Accumulator)
fn rol_acc(cpu: &mut Cpu) -> RnesResult<Cycles> {
    let old_carry = cpu.status.contains(StatusFlags::CARRY);
    let carry = (cpu.a & 0x80) != 0;
    cpu.a = (cpu.a << 1) | if old_carry { 1 } else { 0 };
    cpu.status.set_czn(cpu.a, carry);
    Ok(2)
}

/// ROL - Rotate Left (Memory)
fn rol_mem<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    let value = memory.read_byte(addr)?;
    let old_carry = cpu.status.contains(StatusFlags::CARRY);
    let carry = (value & 0x80) != 0;
    let result = (value << 1) | if old_carry { 1 } else { 0 };
    memory.write_byte(addr, result)?;
    cpu.status.set_czn(result, carry);
    Ok(5)
}

/// ROR - Rotate Right (Accumulator)
fn ror_acc(cpu: &mut Cpu) -> RnesResult<Cycles> {
    let old_carry = cpu.status.contains(StatusFlags::CARRY);
    let carry = (cpu.a & 0x01) != 0;
    cpu.a = (cpu.a >> 1) | if old_carry { 0x80 } else { 0 };
    cpu.status.set_czn(cpu.a, carry);
    Ok(2)
}

/// ROR - Rotate Right (Memory)
fn ror_mem<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let addr = mode.get_address(cpu, memory)?;
    let value = memory.read_byte(addr)?;
    let old_carry = cpu.status.contains(StatusFlags::CARRY);
    let carry = (value & 0x01) != 0;
    let result = (value >> 1) | if old_carry { 0x80 } else { 0 };
    memory.write_byte(addr, result)?;
    cpu.status.set_czn(result, carry);
    Ok(5)
}

/// RTI - Return from Interrupt
fn rti<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M) -> RnesResult<Cycles> {
    // Pull status register
    let status = cpu.pop_byte(memory)?;
    cpu.status = StatusFlags::from_bits(status).unwrap_or_default();
    
    // Pull program counter
    cpu.pc = cpu.pop_word(memory)?;
    
    Ok(6)
}

/// RTS - Return from Subroutine
fn rts<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M) -> RnesResult<Cycles> {
    // Pull program counter
    let addr = cpu.pop_word(memory)?;
    cpu.pc = addr;
    
    Ok(6)
}

/// SBC - Subtract with Carry
fn sbc<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    let operand = mode.get_operand(cpu, memory)?;
    let old_a = cpu.a;
    let carry = cpu.status.contains(StatusFlags::CARRY) as Byte;
    
    // SBC is equivalent to ADC with the operand's complement
    let complemented_operand = operand ^ 0xFF;
    let sum = cpu.a as u16 + complemented_operand as u16 + carry as u16;
    cpu.a = sum as Byte;
    
    // Set flags
    let overflow = ((old_a ^ complemented_operand) & 0x80 == 0) && ((old_a ^ cpu.a) & 0x80 != 0);
    cpu.status.set_czn(cpu.a, sum > 0xFF);
    cpu.status.set_overflow(overflow);
    
    Ok(2)
}

/// SEC - Set Carry Flag
fn sec(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.insert(StatusFlags::CARRY);
    Ok(2)
}

/// SED - Set Decimal Flag
fn sed(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.insert(StatusFlags::DECIMAL);
    Ok(2)
}

/// SEI - Set Interrupt Disable
fn sei(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.status.insert(StatusFlags::INTERRUPT_DISABLE);
    Ok(2)
}

/// STA - Store Accumulator
fn sta<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    mode.write_operand(cpu, memory, cpu.a)?;
    Ok(3)
}

/// STX - Store X Register
fn stx<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    mode.write_operand(cpu, memory, cpu.x)?;
    Ok(3)
}

/// STY - Store Y Register
fn sty<M: MemoryAccess>(cpu: &mut Cpu, memory: &mut M, mode: AddressingMode) -> RnesResult<Cycles> {
    mode.write_operand(cpu, memory, cpu.y)?;
    Ok(3)
}

/// TAX - Transfer Accumulator to X
fn tax(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.x = cpu.a;
    cpu.status.set_zn(cpu.x);
    Ok(2)
}

/// TAY - Transfer Accumulator to Y
fn tay(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.y = cpu.a;
    cpu.status.set_zn(cpu.y);
    Ok(2)
}

/// TSX - Transfer Stack Pointer to X
fn tsx(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.x = cpu.sp;
    cpu.status.set_zn(cpu.x);
    Ok(2)
}

/// TXA - Transfer X to Accumulator
fn txa(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.a = cpu.x;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}

/// TXS - Transfer X to Stack Pointer
fn txs(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.sp = cpu.x;
    Ok(2)
}

/// TYA - Transfer Y to Accumulator
fn tya(cpu: &mut Cpu) -> RnesResult<Cycles> {
    cpu.a = cpu.y;
    cpu.status.set_zn(cpu.a);
    Ok(2)
}
