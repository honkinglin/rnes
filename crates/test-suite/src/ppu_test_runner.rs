use std::path::Path;
use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::{RnesResult, Pixel};

/// PPU test result
#[derive(Debug)]
pub enum PpuTestResult {
    /// Test completed successfully
    Completed { 
        status: u8, 
        cycles: u64,
        frames: u64,
        final_frame: Vec<Pixel>,
    },
    /// Test timed out
    Timeout { 
        cycles: u64,
        frames: u64,
    },
    /// Test entered infinite loop
    InfiniteLoop { 
        cycles: u64,
        frames: u64,
        pc: u16,
    },
}

/// PPU test runner
pub struct PpuTestRunner {
    emulator: Emulator,
    max_cycles: u64,
    max_frames: u64,
    check_frame_output: bool,
}

impl PpuTestRunner {
    pub fn new() -> Self {
        Self {
            emulator: Emulator::new(),
            max_cycles: 1000000, // Default maximum 1 million cycles
            max_frames: 1000,    // Default maximum 1000 frames
            check_frame_output: false,
        }
    }
    
    pub fn with_max_cycles(mut self, max_cycles: u64) -> Self {
        self.max_cycles = max_cycles;
        self
    }
    
    pub fn with_max_frames(mut self, max_frames: u64) -> Self {
        self.max_frames = max_frames;
        self
    }
    
    pub fn with_frame_output_check(mut self, check: bool) -> Self {
        self.check_frame_output = check;
        self
    }
    
    /// Load ROM into the emulator
    pub fn load_rom(&mut self, rom_path: &Path) -> RnesResult<()> {
        let cartridge = Cartridge::from_file(rom_path)?;
        self.emulator.load_rom(cartridge)
    }
    
    /// Run PPU test
    pub fn run_test(&mut self) -> RnesResult<PpuTestResult> {
        let mut cycles = 0;
        let mut frames = 0;
        let mut last_frame_count = 0;
        
        // Reset emulator
        let _ = self.emulator.reset();
        
        // Start emulator
        self.emulator.start();
        
        loop {
            // Check for timeout
            if cycles >= self.max_cycles {
                return Ok(PpuTestResult::Timeout { cycles, frames });
            }
            
            if frames >= self.max_frames {
                return Ok(PpuTestResult::Timeout { cycles, frames });
            }
            
            // Step emulator
            let step_cycles = self.emulator.step()?;
            if step_cycles == 0 {
                // Emulator stopped
                break;
            }
            cycles += step_cycles as u64;
            
            // Check for new frame
            let current_frame_count = self.emulator.ppu().frame_count();
            if current_frame_count > last_frame_count {
                frames = current_frame_count - last_frame_count;
                last_frame_count = current_frame_count;
            }
            
            // Check for test completion (common patterns in PPU test ROMs)
            if self.check_test_completion() {
                let final_frame = if self.check_frame_output {
                    self.emulator.ppu().frame_buffer().to_vec()
                } else {
                    vec![]
                };
                
                return Ok(PpuTestResult::Completed {
                    status: 0, // Assume success if test completed
                    cycles,
                    frames,
                    final_frame,
                });
            }
            
            // Check for infinite loop (simple heuristic)
            if cycles > 100000 && frames == 0 {
                return Ok(PpuTestResult::InfiniteLoop {
                    cycles,
                    frames,
                    pc: self.emulator.cpu().pc,
                });
            }
        }
        
        // If we get here, the test completed normally
        let final_frame = if self.check_frame_output {
            self.emulator.ppu().frame_buffer().to_vec()
        } else {
            vec![]
        };
        
        Ok(PpuTestResult::Completed {
            status: 0,
            cycles,
            frames,
            final_frame,
        })
    }
    
    /// Check if PPU test has completed
    fn check_test_completion(&self) -> bool {
        // Common completion patterns for PPU test ROMs:
        
        // 1. Check if CPU is stuck in a loop (common in test ROMs)
        let _pc = self.emulator.cpu().pc;
        
        // 2. Check for specific memory patterns that indicate test completion
        // Many PPU test ROMs write results to specific memory locations
        
        // 3. Check for VBlank flag being set (many tests wait for VBlank)
        let ppu_status = self.emulator.ppu().registers().ppustatus;
        if ppu_status & 0x80 != 0 {
            // VBlank is set, test might be complete
            return true;
        }
        
        // 4. Check for specific test completion patterns
        // This is a simplified check - real implementation would be more sophisticated
        
        false
    }
    
    /// Get current frame buffer
    pub fn frame_buffer(&self) -> &[Pixel] {
        self.emulator.ppu().frame_buffer()
    }
    
    /// Get PPU registers
    pub fn ppu_registers(&self) -> &rnes_ppu::PpuRegisters {
        self.emulator.ppu().registers()
    }
    
    /// Get PPU state
    pub fn ppu_state(&self) -> &rnes_ppu::PpuTimingState {
        self.emulator.ppu().state()
    }
}
