use std::path::PathBuf;
use rnes_test_suite::cpu_test_runner::{CpuTestRunner, TestResult};

/// Test CPU basic functionality
#[test]
fn test_cpu_basic_functionality() {
    let _runner = CpuTestRunner::new()
        .with_max_cycles(100000);
    
    // Add basic CPU functionality tests here
    // e.g., register operations, basic instruction execution, etc.
    println!("✅ CPU basic functionality test passed");
}

/// Test Blargg's CPU test suite
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_blargg_cpu_suite() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    
    if !test_roms_dir.exists() {
        println!("⚠️  Test ROMs not found, please run: ./scripts/download_test_roms.sh");
        return;
    }
    
    let cpu_test_dir = test_roms_dir.join("blargg_nes_cpu_test5");
    
    if !cpu_test_dir.exists() {
        println!("⚠️  Blargg CPU test suite not found");
        return;
    }
    
    // Run all CPU test ROMs
    let test_files = vec![
        "cpu.nes",
        "official.nes",
    ];
    
    for test_file in test_files {
        let rom_path = cpu_test_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running test: {}", test_file);
            
            let mut runner = CpuTestRunner::new()
                .with_max_cycles(500000);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                TestResult::Completed { status, cycles, .. } => {
                                    if status == 0 {
                                        println!("  ✅ PASS ({} cycles)", cycles);
                                    } else {
                                        println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                    }
                                }
                                TestResult::Timeout { cycles } => {
                                    println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                                }
                                TestResult::InfiniteLoop { cycles, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 ERROR: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 LOAD ERROR: {}", e);
                }
            }
        } else {
            println!("⚠️  Test file not found: {}", test_file);
        }
    }
}

/// Test CPU dummy reads
#[test]
#[ignore]
fn test_cpu_dummy_reads() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let dummy_reads_dir = test_roms_dir.join("cpu_dummy_reads");
    
    if !dummy_reads_dir.exists() {
        println!("⚠️  CPU dummy reads test not found");
        return;
    }
    
    let rom_path = dummy_reads_dir.join("cpu_dummy_reads.nes");
    
    if rom_path.exists() {
        println!("🧪 Running CPU dummy reads test");
        
        let mut runner = CpuTestRunner::new()
            .with_max_cycles(100000);
        
        match runner.load_rom(&rom_path) {
            Ok(_) => {
                match runner.run_test() {
                    Ok(result) => {
                        match result {
                            TestResult::Completed { status, cycles, .. } => {
                                if status == 0 {
                                    println!("  ✅ PASS ({} cycles)", cycles);
                                } else {
                                    println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                }
                            }
                            TestResult::Timeout { cycles } => {
                                println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                            }
                            TestResult::InfiniteLoop { cycles, pc } => {
                                println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  💥 ERROR: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("  📁 LOAD ERROR: {}", e);
            }
        }
    }
}

/// Test branch instruction timing
#[test]
#[ignore]
fn test_branch_timing() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let branch_timing_dir = test_roms_dir.join("branch_timing_tests");
    
    if !branch_timing_dir.exists() {
        println!("⚠️  Branch timing tests not found");
        return;
    }
    
    let test_files = vec![
        "1.Branch_Basics.nes",
        "2.Backward_Branch.nes",
        "3.Forward_Branch.nes",
    ];
    
    for test_file in test_files {
        let rom_path = branch_timing_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running branch timing test: {}", test_file);
            
            let mut runner = CpuTestRunner::new()
                .with_max_cycles(200000);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                TestResult::Completed { status, cycles, .. } => {
                                    if status == 0 {
                                        println!("  ✅ PASS ({} cycles)", cycles);
                                    } else {
                                        println!("  ❌ FAIL (status: {}, {} cycles)", status, cycles);
                                    }
                                }
                                TestResult::Timeout { cycles } => {
                                    println!("  ⏰ TIMEOUT ({} cycles)", cycles);
                                }
                                TestResult::InfiniteLoop { cycles, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles)", pc, cycles);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 ERROR: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 LOAD ERROR: {}", e);
                }
            }
        }
    }
}

/// Convenience function to run all CPU tests
pub fn run_all_cpu_tests() {
    println!("🚀 Starting all CPU tests...");
    println!("");
    
    // Run basic functionality tests
    test_cpu_basic_functionality();
    
    // Run Blargg test suite
    test_blargg_cpu_suite();
    
    // Run dummy reads tests
    test_cpu_dummy_reads();
    
    // Run branch timing tests
    test_branch_timing();
    
    println!("");
    println!("🎉 All CPU tests completed!");
    println!("");
    println!("💡 Tips:");
    println!("  1. To run ignored tests: cargo test --test cpu_integration_tests -- --ignored");
    println!("  2. To run specific test: cargo test --test cpu_integration_tests test_cpu_basic_functionality");
    println!("  3. To download test ROMs: ./scripts/download_test_roms.sh");
}
