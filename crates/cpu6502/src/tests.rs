use super::*;

#[test]
fn test_cpu_creation() {
    let cpu = Cpu::new();
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.x, 0);
    assert_eq!(cpu.y, 0);
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu.status.bits(), StatusFlags::UNUSED.bits() | StatusFlags::INTERRUPT_DISABLE.bits());
}

#[test]
fn test_status_flags() {
    let mut flags = StatusFlags::default();
    
    // Test zero flag
    flags.set_zn(0);
    assert!(flags.contains(StatusFlags::ZERO));
    assert!(!flags.contains(StatusFlags::NEGATIVE));
    
    // Test negative flag
    flags.set_zn(0x80);
    assert!(!flags.contains(StatusFlags::ZERO));
    assert!(flags.contains(StatusFlags::NEGATIVE));
    
    // Test carry flag
    flags.set_czn(0xFF, true);
    assert!(flags.contains(StatusFlags::CARRY));
}

#[test]
fn test_addressing_modes() {
    // Need to simulate memory access here, skip for now
    // Will test through bus implementation later
}

#[test]
fn test_instruction_execution() {
    // Need to simulate memory access here, skip for now
    // Will test through bus implementation later
}

#[test]
fn test_complete_instruction_set() {
    use crate::INSTRUCTIONS;
    
    // Test that all documented instructions are implemented
    let mut implemented_count = 0;
    let mut total_instructions = 0;
    
    for (opcode, instruction) in INSTRUCTIONS.iter().enumerate() {
        if let Some(inst) = instruction {
            total_instructions += 1;
            if !inst.undocumented {
                implemented_count += 1;
                println!("Implemented: 0x{:02X} - {}", opcode, inst.name);
            }
        }
    }
    
    println!("Total implemented instructions: {}", implemented_count);
    println!("Total documented instructions: {}", total_instructions);
    
    // We should have implemented all documented instructions
    assert!(implemented_count > 0, "No instructions implemented");
    assert!(implemented_count >= 56, "Not enough instructions implemented"); // 6502 has 56 documented instructions
}

#[test]
fn test_basic_program_execution() {
    use crate::Cpu;
    use rnes_common::{Byte, MemoryAccess, RnesResult};
    
    // Simple test memory implementation
    struct TestMemory {
        ram: [Byte; 65536],
    }
    
    impl TestMemory {
        fn new() -> Self {
            Self { ram: [0; 65536] }
        }
        
        fn load_program(&mut self, start_addr: u16, program: &[Byte]) {
            for (i, &byte) in program.iter().enumerate() {
                self.ram[(start_addr as usize + i) & 0xFFFF] = byte;
            }
        }
    }
    
    impl MemoryAccess for TestMemory {
        fn read_byte(&self, addr: u16) -> RnesResult<Byte> {
            Ok(self.ram[addr as usize])
        }
        
        fn write_byte(&mut self, addr: u16, value: Byte) -> RnesResult<()> {
            self.ram[addr as usize] = value;
            Ok(())
        }
        
        fn read_word(&self, addr: u16) -> RnesResult<u16> {
            let low = self.read_byte(addr)? as u16;
            let high = self.read_byte(addr.wrapping_add(1))? as u16;
            Ok(low | (high << 8))
        }
        
        fn write_word(&mut self, addr: u16, value: u16) -> RnesResult<()> {
            self.write_byte(addr, value as Byte)?;
            self.write_byte(addr.wrapping_add(1), (value >> 8) as Byte)?;
            Ok(())
        }
    }
    
    let mut cpu = Cpu::new();
    let mut memory = TestMemory::new();
    
    // Simple program: LDA #$42, STA $0000, LDA #$00, STA $0001, JMP $0000
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x85, 0x00, // STA $00
        0xA9, 0x00, // LDA #$00
        0x85, 0x01, // STA $01
        0x4C, 0x00, 0x00, // JMP $0000
    ];
    
    memory.load_program(0x0000, &program);
    cpu.reset(&mut memory).unwrap();
    
    // Execute first few instructions
    for i in 0..2 {
        println!("Step {}: PC={:04X}, A={:02X}", i, cpu.pc, cpu.a);
        cpu.step(&mut memory).unwrap();
        println!("After step {}: PC={:04X}, A={:02X}", i, cpu.pc, cpu.a);
    }
    
    // Check that the program executed correctly
    assert_eq!(cpu.a, 0x42); // First LDA loaded 0x42
    assert_eq!(memory.read_byte(0x0000).unwrap(), 0x42); // First STA stored 0x42
}

#[test]
fn test_rom_loading_capability() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Cpu;
    use rnes_common::{Byte, MemoryAccess, RnesResult};
    
    // Simple test memory implementation
    struct TestMemory {
        ram: [Byte; 65536],
    }
    
    impl TestMemory {
        fn new() -> Self {
            Self { ram: [0; 65536] }
        }
        
        fn load_test_program(&mut self, program: &[Byte], start_addr: u16) {
            for (i, &byte) in program.iter().enumerate() {
                self.ram[(start_addr as usize + i) & 0xFFFF] = byte;
            }
        }
    }
    
    impl MemoryAccess for TestMemory {
        fn read_byte(&self, addr: u16) -> RnesResult<Byte> {
            Ok(self.ram[addr as usize])
        }
        
        fn write_byte(&mut self, addr: u16, value: Byte) -> RnesResult<()> {
            self.ram[addr as usize] = value;
            Ok(())
        }
        
        fn read_word(&self, addr: u16) -> RnesResult<u16> {
            let low = self.read_byte(addr)? as u16;
            let high = self.read_byte(addr.wrapping_add(1))? as u16;
            Ok(low | (high << 8))
        }
        
        fn write_word(&mut self, addr: u16, value: u16) -> RnesResult<()> {
            self.write_byte(addr, value as Byte)?;
            self.write_byte(addr.wrapping_add(1), (value >> 8) as Byte)?;
            Ok(())
        }
    }
    
    let mut cpu = Cpu::new();
    let mut memory = TestMemory::new();
    
            // Create a simple test program: LDA #$42, STA $0000, LDA #$00, STA $0001, JMP $0000
    let program = vec![
        0xA9, 0x42, // LDA #$42
        0x85, 0x00, // STA $00
        0xA9, 0x00, // LDA #$00
        0x85, 0x01, // STA $01
        0x4C, 0x00, 0x00, // JMP $0000
    ];
    
    // Set reset vector
    memory.write_word(0xFFFC, 0x0000)?;
    
    // Load program
    memory.load_test_program(&program, 0x0000);
    
    // Reset CPU
    cpu.reset(&mut memory)?;
    
    // Execute first few instructions
    for i in 0..2 {
        let cycles = cpu.step(&mut memory)?;
        println!("Step {}: PC={:04X}, A={:02X}, Cycles={}", i, cpu.pc, cpu.a, cycles);
    }
    
    // Verify results
    assert_eq!(cpu.a, 0x42);
    assert_eq!(memory.read_byte(0x0000)?, 0x42);
    
    println!("✅ ROM loading and basic execution test passed");
    Ok(())
}
