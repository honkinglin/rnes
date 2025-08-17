use rnes_test_suite::ppu_test_runner::{PpuTestRunner, PpuTestResult};
use std::path::Path;

/// PPU Test Framework Demo
/// Demonstrates how to use the new PPU test runner for testing PPU functionality
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 RNES PPU Test Framework Demo");
    println!("===============================");
    println!("Testing: PPU Test Runner Framework");
    println!("");

    // Test basic PPU functionality without ROM
    println!("🧪 Testing PPU Test Runner Creation");
    let runner = PpuTestRunner::new()
        .with_max_cycles(100000)
        .with_max_frames(100);
    
    println!("✅ PPU Test Runner created successfully");
    println!("");

    // Test with a simple ROM if available
    let test_roms = vec![
        "tests/roms/ppu-tests/blargg_ppu_tests/vbl_clear_time.nes",
        "tests/roms/ppu-tests/full_palette/full_palette.nes",
        "tests/roms/ppu-tests/scrolltest/scroll.nes",
    ];

    for rom_path in test_roms {
        if Path::new(rom_path).exists() {
            println!("🧪 Testing with ROM: {}", rom_path);
            
            let mut runner = PpuTestRunner::new()
                .with_max_cycles(500000)
                .with_max_frames(300)
                .with_frame_output_check(true);
            
            match runner.load_rom(Path::new(rom_path)) {
                Ok(_) => {
                    println!("  ✅ ROM loaded successfully");
                    
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                PpuTestResult::Completed { cycles, frames, final_frame, status: _ } => {
                                    println!("  ✅ Test completed ({} cycles, {} frames)", cycles, frames);
                                    
                                    if !final_frame.is_empty() {
                                        println!("  📊 Frame buffer size: {} pixels", final_frame.len());
                                        
                                        // Analyze frame content
                                        let non_black_pixels = final_frame.iter()
                                            .filter(|&&p| p != rnes_common::Pixel::BLACK)
                                            .count();
                                        println!("  📊 Non-black pixels: {}", non_black_pixels);
                                        
                                        if non_black_pixels > 0 {
                                            println!("  ✅ Frame contains visible content");
                                        } else {
                                            println!("  ⚠️  Frame appears to be blank");
                                        }
                                    }
                                }
                                PpuTestResult::Timeout { cycles, frames } => {
                                    println!("  ⏰ Test timed out ({} cycles, {} frames)", cycles, frames);
                                }
                                PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                    println!("  🔄 Test entered infinite loop at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
                                }
                            }
                        }
                        Err(e) => {
                            println!("  💥 Test error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  📁 ROM load error: {}", e);
                }
            }
        } else {
            println!("⏭️  Skipping: {} (file not found)", rom_path);
        }
        println!("");
    }

    println!("🎉 PPU Test Framework Demo completed!");
    println!("");
    println!("💡 To run full PPU tests:");
    println!("   1. Download test ROMs: ./scripts/download_ppu_test_roms.sh");
    println!("   2. Run tests: cargo test -p rnes-test-suite --test ppu_integration_tests -- --ignored");

    Ok(())
}
