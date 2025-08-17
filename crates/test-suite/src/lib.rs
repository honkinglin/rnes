//! NES Emulator Test Suite
//! 
//! This crate contains all integration tests and testing utilities

pub mod cpu_test_runner;
pub mod ppu_test_runner;

pub use cpu_test_runner::CpuTestRunner;
pub use ppu_test_runner::{PpuTestRunner, PpuTestResult};

// Include test modules
#[cfg(test)]
mod m2_integration_tests;
