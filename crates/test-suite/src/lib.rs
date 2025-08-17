//! NES Emulator Test Suite
//! 
//! This crate contains all integration tests and testing utilities

pub mod cpu_test_runner;
pub mod ppu_test_runner;
pub mod apu_test_runner;

pub use cpu_test_runner::CpuTestRunner;
pub use ppu_test_runner::{PpuTestRunner, PpuTestResult};
pub use apu_test_runner::{ApuTestRunner, ApuTestResult};

// Test modules are now separate test targets
