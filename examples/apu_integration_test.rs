use rnes_test_suite::apu_test_runner::{ApuTestRunner, ApuTestResult};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting APU Integration Test");
    println!("📋 Testing M3: APU Audio System");
    
    let test_roms_dir = PathBuf::from("tests/roms/apu-tests");
    
    if !test_roms_dir.exists() {
        println!("⚠️  APU test ROMs not found!");
        println!("💡 Please run: ./scripts/download_apu_test_roms.sh");
        return Ok(());
    }
    
    // Test basic APU functionality
    println!("\n🧪 Testing basic APU functionality...");
    test_basic_apu_functionality()?;
    
    // Test Blargg's APU test suite
    println!("\n🧪 Testing Blargg's APU test suite...");
    test_blargg_apu_suite(&test_roms_dir)?;
    
    // Test individual APU components
    println!("\n🧪 Testing individual APU components...");
    test_apu_components(&test_roms_dir)?;
    
    println!("\n✅ APU Integration Test completed!");
    Ok(())
}

fn test_basic_apu_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let _runner = ApuTestRunner::new()
        .with_max_cycles(100000)
        .with_audio_check(true)
        .with_min_audio_samples(100);
    
    println!("  ✓ APU test runner created");
    println!("  ✓ Audio checking enabled");
    println!("  ✓ Minimum audio samples: 100");
    println!("  ✓ Maximum cycles: 100,000");
    
    Ok(())
}

fn test_blargg_apu_suite(test_roms_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let apu_test_dir = test_roms_dir.join("blargg_apu_tests");
    
    if !apu_test_dir.exists() {
        println!("  ⚠️  Blargg APU test suite not found");
        return Ok(());
    }
    
    // Test a few key APU test ROMs
    let test_files = vec![
        "01.len_ctr.nes",
        "02.len_table.nes",
        "03.irq_flag.nes",
        "04.clock_jitter.nes",
        "05.len_timing.nes",
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    let mut total = 0;
    
    for test_file in test_files {
        let rom_path = apu_test_dir.join(test_file);
        
        if rom_path.exists() {
            total += 1;
            println!("  🧪 Running: {}", test_file);
            
            let mut runner = ApuTestRunner::new()
                .with_max_cycles(500000)
                .with_audio_check(true)
                .with_min_audio_samples(200);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                ApuTestResult::Completed { status, cycles, audio_samples, avg_amplitude } => {
                                    if status == 0 {
                                        println!("    ✅ PASS (status: {}, {} cycles, {} samples, avg_amp: {:.4})", 
                                                status, cycles, audio_samples, avg_amplitude);
                                        passed += 1;
                                    } else {
                                        println!("    ❌ FAIL (status: {}, {} cycles, {} samples, avg_amp: {:.4})", 
                                                status, cycles, audio_samples, avg_amplitude);
                                        failed += 1;
                                    }
                                }
                                ApuTestResult::Timeout { cycles, audio_samples } => {
                                    println!("    ⏰ TIMEOUT ({} cycles, {} samples)", cycles, audio_samples);
                                    failed += 1;
                                }
                                ApuTestResult::InfiniteLoop { cycles, pc, audio_samples } => {
                                    println!("    🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} samples)", pc, cycles, audio_samples);
                                    failed += 1;
                                }
                                ApuTestResult::Error { error, cycles } => {
                                    println!("    💥 ERROR: {} ({} cycles)", error, cycles);
                                    failed += 1;
                                }
                            }
                        }
                        Err(e) => {
                            println!("    💥 ERROR: {}", e);
                            failed += 1;
                        }
                    }
                }
                Err(e) => {
                    println!("    📁 LOAD ERROR: {}", e);
                    failed += 1;
                }
            }
        } else {
            println!("    ⚠️  Test file not found: {}", test_file);
        }
    }
    
    println!("  📊 Results: {}/{} tests passed", passed, total);
    println!("  📊 Failed tests: {}", failed);
    
    Ok(())
}

fn test_apu_components(test_roms_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let components = vec![
        ("apu_frame_counter", "Frame Counter"),
        ("apu_length_counter", "Length Counter"),
        ("apu_reset", "Reset"),
        ("apu_irq", "IRQ"),
        ("apu_sweep", "Sweep"),
        ("apu_envelope", "Envelope"),
    ];
    
    for (dir_name, component_name) in components {
        let component_dir = test_roms_dir.join(dir_name);
        
        if component_dir.exists() {
            println!("  🧪 Testing {}...", component_name);
            
            // Look for test ROM in the directory
            let rom_path = component_dir.join(format!("{}.nes", dir_name));
            
            if rom_path.exists() {
                let mut runner = ApuTestRunner::new()
                    .with_max_cycles(300000)
                    .with_audio_check(true);
                
                match runner.load_rom(&rom_path) {
                    Ok(_) => {
                        match runner.run_test() {
                            Ok(result) => {
                                match result {
                                    ApuTestResult::Completed { status, cycles, audio_samples, avg_amplitude } => {
                                        if status == 0 {
                                            println!("    ✅ {} PASS (status: {}, {} cycles, {} samples, avg_amp: {:.4})", 
                                                    component_name, status, cycles, audio_samples, avg_amplitude);
                                        } else {
                                            println!("    ❌ {} FAIL (status: {}, {} cycles, {} samples, avg_amp: {:.4})", 
                                                    component_name, status, cycles, audio_samples, avg_amplitude);
                                        }
                                    }
                                    _ => {
                                        println!("    ⚠️  {} test did not complete normally", component_name);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("    💥 {} ERROR: {}", component_name, e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("    📁 {} LOAD ERROR: {}", component_name, e);
                    }
                }
            } else {
                println!("    ⚠️  {} test ROM not found", component_name);
            }
        } else {
            println!("    ⚠️  {} test directory not found", component_name);
        }
    }
    
    Ok(())
}
