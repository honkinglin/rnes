use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::{RnesResult, AudioSample};
use std::path::Path;
use std::time::Duration;

/// APU test result
#[derive(Debug, Clone)]
pub enum ApuTestResult {
    /// Test completed successfully
    Completed {
        status: u8,
        cycles: u32,
        audio_samples: usize,
        avg_amplitude: f32,
    },
    /// Test timed out
    Timeout {
        cycles: u32,
        audio_samples: usize,
    },
    /// Test entered infinite loop
    InfiniteLoop {
        cycles: u32,
        pc: u16,
        audio_samples: usize,
    },
    /// Test failed with error
    Error {
        error: String,
        cycles: u32,
    },
}

/// APU test runner
pub struct ApuTestRunner {
    emulator: Emulator,
    max_cycles: u32,
    timeout_duration: Duration,
    check_audio: bool,
    min_audio_samples: usize,
    max_audio_amplitude: f32,
}

impl ApuTestRunner {
    /// Create new APU test runner
    pub fn new() -> Self {
        Self {
            emulator: Emulator::new(),
            max_cycles: 1_000_000,
            timeout_duration: Duration::from_secs(30),
            check_audio: true,
            min_audio_samples: 1000,
            max_audio_amplitude: 0.1,
        }
    }

    /// Set maximum cycles to run
    pub fn with_max_cycles(mut self, max_cycles: u32) -> Self {
        self.max_cycles = max_cycles;
        self
    }

    /// Set timeout duration
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout_duration = timeout;
        self
    }

    /// Enable/disable audio checking
    pub fn with_audio_check(mut self, check_audio: bool) -> Self {
        self.check_audio = check_audio;
        self
    }

    /// Set minimum audio samples required
    pub fn with_min_audio_samples(mut self, min_samples: usize) -> Self {
        self.min_audio_samples = min_samples;
        self
    }

    /// Set maximum audio amplitude threshold
    pub fn with_max_audio_amplitude(mut self, max_amplitude: f32) -> Self {
        self.max_audio_amplitude = max_amplitude;
        self
    }

    /// Load ROM from path
    pub fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) -> RnesResult<()> {
        let cartridge = Cartridge::from_file(rom_path)?;
        self.emulator.load_rom(cartridge)?;
        Ok(())
    }

    /// Run APU test
    pub fn run_test(&mut self) -> RnesResult<ApuTestResult> {
        let start_time = std::time::Instant::now();
        let mut cycles = 0;
        let mut audio_samples = Vec::new();
        let mut last_pc = 0u16;
        let mut pc_repeat_count = 0;
        const PC_REPEAT_THRESHOLD: u32 = 1000;

        // Start emulator
        self.emulator.start();

        while cycles < self.max_cycles {
            // Check timeout
            if start_time.elapsed() > self.timeout_duration {
                return Ok(ApuTestResult::Timeout {
                    cycles,
                    audio_samples: audio_samples.len(),
                });
            }

            // Step emulator
            let step_cycles = self.emulator.step()?;
            if step_cycles == 0 {
                break;
            }
            cycles += step_cycles;

            // Check for infinite loop
            let current_pc = self.emulator.cpu().pc;
            if current_pc == last_pc {
                pc_repeat_count += 1;
                if pc_repeat_count > PC_REPEAT_THRESHOLD {
                    return Ok(ApuTestResult::InfiniteLoop {
                        cycles,
                        pc: current_pc,
                        audio_samples: audio_samples.len(),
                    });
                }
            } else {
                pc_repeat_count = 0;
                last_pc = current_pc;
            }

            // Collect audio samples
            if self.check_audio {
                let samples = self.emulator.get_audio_samples();
                audio_samples.extend(samples);
            }

            // Check if test completed (look for specific patterns in memory)
            if self.check_test_completion() {
                let status = self.get_test_status();
                let avg_amplitude = self.calculate_average_amplitude(&audio_samples);
                
                return Ok(ApuTestResult::Completed {
                    status,
                    cycles,
                    audio_samples: audio_samples.len(),
                    avg_amplitude,
                });
            }
        }

        // Test timed out
        Ok(ApuTestResult::Timeout {
            cycles,
            audio_samples: audio_samples.len(),
        })
    }

    /// Check if test has completed
    fn check_test_completion(&mut self) -> bool {
        // Check for common test completion patterns
        // This is a simplified check - real tests might have more complex completion detection
        
        // Check if CPU is in a known halt state
        let cpu = self.emulator.cpu();
        let pc = cpu.pc;
        
        // Common test completion addresses
        let completion_addresses = [
            0x8000, // Common test start address
            0x8001, // Test completion address
            0x8002, // Alternative completion
        ];
        
        completion_addresses.contains(&pc)
    }

    /// Get test status from memory
    fn get_test_status(&mut self) -> u8 {
        // Read status from common test status locations
        // This is a simplified implementation
        
        // Try to read status from 0x6000 (common test status location)
        if let Ok(status) = self.emulator.bus_mut().read_byte(0x6000) {
            return status;
        }
        
        // Fallback: check APU status register
        if let Ok(status) = self.emulator.bus_mut().read_byte(0x4015) {
            return status;
        }
        
        0 // Default status
    }

    /// Calculate average audio amplitude
    fn calculate_average_amplitude(&self, samples: &[AudioSample]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        
        let sum: f32 = samples.iter().map(|&s| s.abs()).sum();
        sum / samples.len() as f32
    }

    /// Validate audio output
    pub fn validate_audio(&self, result: &ApuTestResult) -> bool {
        match result {
            ApuTestResult::Completed { audio_samples, avg_amplitude, .. } => {
                // Check if we have enough audio samples
                if *audio_samples < self.min_audio_samples {
                    println!("⚠️  Insufficient audio samples: {} < {}", audio_samples, self.min_audio_samples);
                    return false;
                }
                
                // Check if audio amplitude is reasonable
                if *avg_amplitude > self.max_audio_amplitude {
                    println!("⚠️  Audio amplitude too high: {} > {}", avg_amplitude, self.max_audio_amplitude);
                    return false;
                }
                
                true
            }
            _ => false,
        }
    }

    /// Get emulator reference
    pub fn emulator(&self) -> &Emulator {
        &self.emulator
    }

    /// Get mutable emulator reference
    pub fn emulator_mut(&mut self) -> &mut Emulator {
        &mut self.emulator
    }
}

impl Default for ApuTestRunner {
    fn default() -> Self {
        Self::new()
    }
}
