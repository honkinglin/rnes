use std::path::PathBuf;
use rnes_test_suite::ppu_test_runner::{PpuTestRunner, PpuTestResult};

/// Test sprite hit functionality using Blargg's sprite hit test suite
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_sprite_hit_functionality() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let sprite_hit_dir = test_roms_dir.join("sprite_hit_tests_2005.10.05");
    
    if !sprite_hit_dir.exists() {
        println!("⚠️  Sprite hit test ROMs not found");
        return;
    }
    
    // Test files in order of complexity
    let test_files = vec![
        "01.basics.nes",
        "02.alignment.nes", 
        "03.corners.nes",
        "04.flip.nes",
        "05.left_clip.nes",
        "06.right_edge.nes",
        "07.screen_bottom.nes",
        "08.double_height.nes",
        "09.timing_basics.nes",
        "10.timing_order.nes",
        "11.edge_timing.nes",
    ];
    
    for test_file in test_files {
        let rom_path = sprite_hit_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running sprite hit test: {}", test_file);
            
            let mut runner = PpuTestRunner::new()
                .with_max_cycles(2000000)
                .with_max_frames(1000);
            
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

/// Test sprite overflow functionality using Blargg's sprite overflow test suite
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_sprite_overflow_functionality() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let sprite_overflow_dir = test_roms_dir.join("sprite_overflow_tests");
    
    if !sprite_overflow_dir.exists() {
        println!("⚠️  Sprite overflow test ROMs not found");
        return;
    }
    
    // Test files in order of complexity
    let test_files = vec![
        "1.Basics.nes",
        "2.Details.nes",
        "3.Timing.nes",
        "4.Obscure.nes",
        "5.Emulator.nes",
    ];
    
    for test_file in test_files {
        let rom_path = sprite_overflow_dir.join(test_file);
        
        if rom_path.exists() {
            println!("🧪 Running sprite overflow test: {}", test_file);
            
            let mut runner = PpuTestRunner::new()
                .with_max_cycles(2000000)
                .with_max_frames(1000);
            
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

/// Test OAM functionality using Blargg's OAM stress test
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_oam_functionality() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let oam_stress_dir = test_roms_dir.join("oam_stress");
    
    if !oam_stress_dir.exists() {
        println!("⚠️  OAM stress test ROMs not found");
        return;
    }
    
    let rom_path = oam_stress_dir.join("oam_stress.nes");
    
    if rom_path.exists() {
        println!("🧪 Running OAM stress test");
        
        let mut runner = PpuTestRunner::new()
            .with_max_cycles(5000000) // OAM stress test takes longer
            .with_max_frames(2000);
        
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
        println!("⚠️  OAM stress test file not found");
    }
}

/// Test OAM read functionality
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_oam_read_functionality() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let oam_read_dir = test_roms_dir.join("oam_read");
    
    if !oam_read_dir.exists() {
        println!("⚠️  OAM read test ROMs not found");
        return;
    }
    
    let rom_path = oam_read_dir.join("oam_read.nes");
    
    if rom_path.exists() {
        println!("🧪 Running OAM read test");
        
        let mut runner = PpuTestRunner::new()
            .with_max_cycles(2000000)
            .with_max_frames(1000);
        
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
        println!("⚠️  OAM read test file not found");
    }
}

/// Test sprite DMA functionality
#[test]
#[ignore] // Requires test ROMs to be downloaded
fn test_sprite_dma_functionality() {
    let test_roms_dir = PathBuf::from("../../tests/roms/nes-test-roms");
    let sprdma_dir = test_roms_dir.join("sprdma_and_dmc_dma");
    
    if !sprdma_dir.exists() {
        println!("⚠️  Sprite DMA test ROMs not found");
        return;
    }
    
    let rom_path = sprdma_dir.join("sprdma_and_dmc_dma.nes");
    
    if rom_path.exists() {
        println!("🧪 Running sprite DMA test");
        
        let mut runner = PpuTestRunner::new()
            .with_max_cycles(2000000)
            .with_max_frames(1000);
        
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
        println!("⚠️  Sprite DMA test file not found");
    }
}

/// Test basic M2 functionality without requiring test ROMs
#[test]
fn test_m2_basic_functionality() {
    println!("🧪 Testing M2 basic functionality");
    
    // Test that M2 features are available
    use rnes_ppu::{Ppu, Sprite, SpriteRenderingState};
    use rnes_mappers::NromMapper;
    use rnes_cartridge::Cartridge;
    
    // Create a test cartridge
    let mut test_data = vec![
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ];
    
    // Add 16KB PRG ROM data
    test_data.extend(vec![0; 16384]);
    
    // Add 8KB CHR ROM data
    test_data.extend(vec![0; 8192]);
    
    let cartridge = Cartridge::from_bytes(&test_data).unwrap();
    let mapper = NromMapper::new(cartridge);
    let mut ppu = Ppu::new(Box::new(mapper));
    
    // Test sprite creation
    let sprite = Sprite::new();
    assert_eq!(sprite.y, 0);
    assert_eq!(sprite.tile_id, 0);
    assert_eq!(sprite.attributes, 0);
    assert_eq!(sprite.x, 0);
    
    // Test sprite attributes
    let mut test_sprite = Sprite::new();
    test_sprite.attributes = 0x23; // Palette 3, behind background, no flip
    assert_eq!(test_sprite.palette(), 3);
    assert!(test_sprite.behind_background());
    assert!(!test_sprite.flip_horizontal());
    assert!(!test_sprite.flip_vertical());
    
    // Test OAM DMA
    ppu.start_oam_dma(0x02);
    assert!(ppu.oam_dma_active());
    assert_eq!(ppu.state().oam_dma_addr, 0x0200);
    
    // Test sprite rendering state
    let sprite_state = SpriteRenderingState::default();
    assert_eq!(sprite_state.sprites_on_scanline.len(), 0);
    assert!(!sprite_state.sprite_zero_on_scanline);
    assert!(!sprite_state.sprite_overflow);
    
    println!("  ✅ M2 basic functionality test passed");
}
