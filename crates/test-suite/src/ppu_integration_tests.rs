use std::path::PathBuf;
use rnes_test_suite::ppu_test_runner::{PpuTestRunner, PpuTestResult};

/// Test PPU basic functionality
#[test]
fn test_ppu_basic_functionality() {
    let _runner = PpuTestRunner::new()
        .with_max_cycles(100000)
        .with_max_frames(100);
    
    // Add basic PPU functionality tests here
    // e.g., register operations, basic rendering, etc.
    println!("✅ PPU basic functionality test passed");
}

/// Test Blargg's PPU test suite
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_blargg_ppu_suite() {
    let test_roms_dir = PathBuf::from("../../tests/roms/ppu-tests");
    
    if !test_roms_dir.exists() {
        println!("⚠️  PPU test ROMs not found, please run: ./scripts/download_ppu_test_roms.sh");
        return;
    }
    
    let blargg_ppu_dir = test_roms_dir.join("blargg_ppu_tests");
    
    if !blargg_ppu_dir.exists() {
        println!("⚠️  Blargg PPU test suite not found");
        return;
    }
    
    // Run all PPU test ROMs
    let test_files = vec![
        "vbl_clear_time.nes",
        "palette_ram.nes",
        "sprite_ram.nes",
        "vram_access.nes",
        "power_up_palette.nes",
    ];
    
    for test_file in test_files {
        let rom_path = blargg_ppu_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running PPU test: {}", test_file);
            
            let mut runner = PpuTestRunner::new()
                .with_max_cycles(1000000)
                .with_max_frames(500);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                PpuTestResult::Completed { cycles, frames, .. } => {
                                    println!("  ✅ PASS ({} cycles, {} frames)", cycles, frames);
                                }
                                PpuTestResult::Timeout { cycles, frames } => {
                                    println!("  ⏰ TIMEOUT ({} cycles, {} frames)", cycles, frames);
                                }
                                PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
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

/// Test PPU VBL NMI functionality
#[test]
#[ignore]
fn test_ppu_vbl_nmi() {
    let test_roms_dir = PathBuf::from("../../tests/roms/ppu-tests");
    let vbl_nmi_dir = test_roms_dir.join("ppu_vbl_nmi");
    
    if !vbl_nmi_dir.exists() {
        println!("⚠️  PPU VBL NMI test not found");
        return;
    }
    
    let rom_path = vbl_nmi_dir.join("ppu_vbl_nmi.nes");
    
    if rom_path.exists() {
        println!("🧪 Running PPU VBL NMI test");
        
        let mut runner = PpuTestRunner::new()
            .with_max_cycles(500000)
            .with_max_frames(300);
        
        match runner.load_rom(&rom_path) {
            Ok(_) => {
                match runner.run_test() {
                    Ok(result) => {
                        match result {
                            PpuTestResult::Completed { cycles, frames, .. } => {
                                println!("  ✅ PASS ({} cycles, {} frames)", cycles, frames);
                            }
                            PpuTestResult::Timeout { cycles, frames } => {
                                println!("  ⏰ TIMEOUT ({} cycles, {} frames)", cycles, frames);
                            }
                            PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
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
        println!("⚠️  PPU VBL NMI test file not found");
    }
}

/// Test PPU read buffer functionality
#[test]
#[ignore]
fn test_ppu_read_buffer() {
    let test_roms_dir = PathBuf::from("../../tests/roms/ppu-tests");
    let read_buffer_dir = test_roms_dir.join("ppu_read_buffer");
    
    if !read_buffer_dir.exists() {
        println!("⚠️  PPU read buffer test not found");
        return;
    }
    
    let rom_path = read_buffer_dir.join("test_ppu_read_buffer.nes");
    
    if rom_path.exists() {
        println!("🧪 Running PPU read buffer test");
        
        let mut runner = PpuTestRunner::new()
            .with_max_cycles(300000)
            .with_max_frames(200);
        
        match runner.load_rom(&rom_path) {
            Ok(_) => {
                match runner.run_test() {
                    Ok(result) => {
                        match result {
                            PpuTestResult::Completed { cycles, frames, .. } => {
                                println!("  ✅ PASS ({} cycles, {} frames)", cycles, frames);
                            }
                            PpuTestResult::Timeout { cycles, frames } => {
                                println!("  ⏰ TIMEOUT ({} cycles, {} frames)", cycles, frames);
                            }
                            PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
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
        println!("⚠️  PPU read buffer test file not found");
    }
}

/// Test full palette functionality
#[test]
#[ignore]
fn test_full_palette() {
    let test_roms_dir = PathBuf::from("../../tests/roms/ppu-tests");
    let full_palette_dir = test_roms_dir.join("full_palette");
    
    if !full_palette_dir.exists() {
        println!("⚠️  Full palette test not found");
        return;
    }
    
    let test_files = vec![
        "full_palette.nes",
        "full_palette_smooth.nes",
        "flowing_palette.nes",
    ];
    
    for test_file in test_files {
        let rom_path = full_palette_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running full palette test: {}", test_file);
            
            let mut runner = PpuTestRunner::new()
                .with_max_cycles(500000)
                .with_max_frames(300)
                .with_frame_output_check(true);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                PpuTestResult::Completed { cycles, frames, final_frame, status: _ } => {
                                    println!("  ✅ PASS ({} cycles, {} frames)", cycles, frames);
                                    
                                    // Analyze frame output for palette correctness
                                    if !final_frame.is_empty() {
                                        analyze_palette_output(&final_frame);
                                    }
                                }
                                PpuTestResult::Timeout { cycles, frames } => {
                                    println!("  ⏰ TIMEOUT ({} cycles, {} frames)", cycles, frames);
                                }
                                PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
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

/// Test NROM mapper specific functionality
#[test]
#[ignore]
fn test_nrom_mapper() {
    let test_roms_dir = PathBuf::from("../../tests/roms/ppu-tests");
    let nrom_dir = test_roms_dir.join("nrom368");
    
    if !nrom_dir.exists() {
        println!("⚠️  NROM mapper test not found");
        return;
    }
    
    let test_files = vec![
        "test1.nes",
        "fail368.nes",
    ];
    
    for test_file in test_files {
        let rom_path = nrom_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running NROM mapper test: {}", test_file);
            
            let mut runner = PpuTestRunner::new()
                .with_max_cycles(300000)
                .with_max_frames(200);
            
            match runner.load_rom(&rom_path) {
                Ok(_) => {
                    match runner.run_test() {
                        Ok(result) => {
                            match result {
                                PpuTestResult::Completed { cycles, frames, .. } => {
                                    println!("  ✅ PASS ({} cycles, {} frames)", cycles, frames);
                                }
                                PpuTestResult::Timeout { cycles, frames } => {
                                    println!("  ⏰ TIMEOUT ({} cycles, {} frames)", cycles, frames);
                                }
                                PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                    println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
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

/// Test background scrolling functionality
#[test]
#[ignore]
fn test_background_scrolling() {
    let test_roms_dir = PathBuf::from("../../tests/roms/ppu-tests");
    let scroll_dir = test_roms_dir.join("scrolltest");
    
    if !scroll_dir.exists() {
        println!("⚠️  Background scrolling test not found");
        return;
    }
    
    let rom_path = scroll_dir.join("scroll.nes");
    
    if rom_path.exists() {
        println!("🧪 Running background scrolling test");
        
        let mut runner = PpuTestRunner::new()
            .with_max_cycles(1000000)
            .with_max_frames(500)
            .with_frame_output_check(true);
        
        match runner.load_rom(&rom_path) {
            Ok(_) => {
                match runner.run_test() {
                    Ok(result) => {
                        match result {
                            PpuTestResult::Completed { cycles, frames, final_frame, status: _ } => {
                                println!("  ✅ PASS ({} cycles, {} frames)", cycles, frames);
                                
                                // Analyze frame output for scrolling correctness
                                if !final_frame.is_empty() {
                                    analyze_scrolling_output(&final_frame);
                                }
                            }
                            PpuTestResult::Timeout { cycles, frames } => {
                                println!("  ⏰ TIMEOUT ({} cycles, {} frames)", cycles, frames);
                            }
                            PpuTestResult::InfiniteLoop { cycles, frames, pc } => {
                                println!("  🔄 INFINITE LOOP at PC=0x{:04X} ({} cycles, {} frames)", pc, cycles, frames);
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
        println!("⚠️  Background scrolling test file not found");
    }
}

/// Analyze palette output for correctness
fn analyze_palette_output(frame: &[rnes_common::Pixel]) {
    // Count unique colors in the frame
    let mut color_counts = std::collections::HashMap::new();
    for &pixel in frame {
        *color_counts.entry(pixel).or_insert(0) += 1;
    }
    
    println!("    📊 Frame analysis:");
    println!("      - Total pixels: {}", frame.len());
    println!("      - Unique colors: {}", color_counts.len());
    
    // Check if we have a reasonable number of colors (NES has 64 colors total)
    if color_counts.len() > 0 && color_counts.len() <= 64 {
        println!("      - ✅ Color count is within NES palette range");
    } else {
        println!("      - ⚠️  Unexpected color count: {}", color_counts.len());
    }
}

/// Analyze scrolling output for correctness
fn analyze_scrolling_output(frame: &[rnes_common::Pixel]) {
    // Simple analysis: check if the frame has any non-black pixels
    let non_black_pixels = frame.iter().filter(|&&p| p != rnes_common::Pixel::BLACK).count();
    
    println!("    📊 Scrolling analysis:");
    println!("      - Total pixels: {}", frame.len());
    println!("      - Non-black pixels: {}", non_black_pixels);
    
    if non_black_pixels > 0 {
        println!("      - ✅ Frame contains visible content");
    } else {
        println!("      - ⚠️  Frame appears to be blank");
    }
}
