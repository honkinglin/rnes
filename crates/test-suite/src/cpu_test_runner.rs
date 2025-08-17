use std::fs;
use std::path::Path;
use rnes_cpu6502::Cpu;
use rnes_common::{Byte, MemoryAccess, RnesResult};

/// Simple test memory implementation
struct TestMemory {
    ram: [Byte; 65536],
    rom_data: Vec<Byte>,
}

impl TestMemory {
    fn new() -> Self {
        Self {
            ram: [0; 65536],
            rom_data: Vec::new(),
        }
    }
    
    fn load_rom(&mut self, rom_path: &Path) -> RnesResult<()> {
        let rom_data = fs::read(rom_path)?;
        
        // Simple NES ROM loading (skip iNES header)
        let rom_start = if rom_data.len() > 16 && rom_data[0] == 0x4E && rom_data[1] == 0x45 && rom_data[2] == 0x53 {
            // iNES format, skip 16-byte header
            &rom_data[16..]
        } else {
            &rom_data
        };
        
        self.rom_data = rom_start.to_vec();
        
        // Load ROM data into memory
        for (i, &byte) in rom_start.iter().enumerate() {
            let addr = 0x8000 + i;
            if addr < 0x10000 {
                self.ram[addr] = byte;
            }
        }
        
        // Set up reset vector if not already set
        if self.ram[0xFFFC] == 0 && self.ram[0xFFFD] == 0 {
            // Set reset vector to point to ROM start
            self.ram[0xFFFC] = 0x00; // Low byte
            self.ram[0xFFFD] = 0x80; // High byte (ROM starts at 0x8000)
        }
        
        Ok(())
    }
    
    #[allow(dead_code)]
    fn reset_vector(&self) -> u16 {
        // Read reset vector (0xFFFC-0xFFFD)
        let low = self.ram[0xFFFC] as u16;
        let high = self.ram[0xFFFD] as u16;
        low | (high << 8)
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

/// CPU test runner
pub struct CpuTestRunner {
    cpu: Cpu,
    memory: TestMemory,
    max_cycles: u64,
}

impl CpuTestRunner {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: TestMemory::new(),
            max_cycles: 1000000, // Default maximum 1 million cycles
        }
    }
    
    pub fn with_max_cycles(mut self, max_cycles: u64) -> Self {
        self.max_cycles = max_cycles;
        self
    }
    
    pub fn load_rom(&mut self, rom_path: &Path) -> RnesResult<()> {
        self.memory.load_rom(rom_path)?;
        Ok(())
    }
    
    pub fn run_test(&mut self) -> RnesResult<TestResult> {
        // Reset CPU
        self.cpu.reset(&mut self.memory)?;
        
        let mut cycles = 0;
        let mut last_pc = self.cpu.pc;
        let mut stall_count = 0;
        
        // Run test until maximum cycles or infinite loop detected
        while cycles < self.max_cycles {
            let step_cycles = self.cpu.step(&mut self.memory)?;
            cycles += step_cycles as u64;
            
            // Detect infinite loop
            if self.cpu.pc == last_pc {
                stall_count += 1;
                if stall_count > 1000 {
                    return Ok(TestResult::InfiniteLoop {
                        cycles,
                        pc: self.cpu.pc,
                    });
                }
            } else {
                stall_count = 0;
                last_pc = self.cpu.pc;
            }
            
            // Check test completion flag (usually at 0x6000)
            if let Ok(status) = self.memory.read_byte(0x6000) {
                if status != 0 {
                    return Ok(TestResult::Completed {
                        cycles,
                        status,
                        cpu_state: self.cpu.clone(),
                    });
                }
            }
        }
        
        Ok(TestResult::Timeout { cycles })
    }
    
    pub fn get_cpu_state(&self) -> &Cpu {
        &self.cpu
    }
}

#[derive(Debug)]
pub enum TestResult {
    Completed {
        cycles: u64,
        status: Byte,
        cpu_state: Cpu,
    },
    Timeout {
        cycles: u64,
    },
    InfiniteLoop {
        cycles: u64,
        pc: u16,
    },
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        matches!(self, TestResult::Completed { status, .. } if *status == 0)
    }
    
    pub fn get_status_message(&self) -> &'static str {
        match self {
            TestResult::Completed { status, .. } => match *status {
                0 => "PASS",
                1 => "FAIL",
                2 => "TIMEOUT",
                _ => "UNKNOWN",
            },
            TestResult::Timeout { .. } => "TIMEOUT",
            TestResult::InfiniteLoop { .. } => "INFINITE LOOP",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_rom_loading() {
        let _runner = CpuTestRunner::new();
        // Add basic ROM loading tests here
    }
}
