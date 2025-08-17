//! NES Emulator Test Suite
//! 
//! This crate contains all integration tests and testing utilities

pub mod cpu_test_runner;
pub mod ppu_test_runner;
pub mod apu_test_runner;
pub mod m4_test_runner;
pub mod m5_test_runner;
pub mod m6_integration_tests;

pub use cpu_test_runner::CpuTestRunner;
pub use ppu_test_runner::{PpuTestRunner, PpuTestResult};
pub use apu_test_runner::{ApuTestRunner, ApuTestResult};
pub use m4_test_runner::{M4TestRunner, M4TestResult};
pub use m5_test_runner::{M5TestRunner, M5TestResult};
pub use m6_integration_tests::M6TestRunner;

// Test modules are now separate test targets
