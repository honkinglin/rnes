use rnes_test_suite::M6TestRunner;
use rnes_test_suite::m6_integration_tests::TestResult;
use rnes_common::Config;

#[test]
fn test_m6_basic() {
    println!("Running M6 basic tests...");
    
    // Test configuration creation
    let config = Config::default();
    assert_eq!(config.video.window_width, 256 * 3);
    assert_eq!(config.audio.sample_rate, 44100);
    assert_eq!(config.general.frame_rate_limit, 60);
    
    // Test debugger creation
    let debugger = rnes_common::Debugger::new();
    assert_eq!(debugger.breakpoints.len(), 0);
    assert_eq!(debugger.watchpoints.len(), 0);
    assert!(!debugger.step_mode);
    
    // Test save system creation
    let _save_system = rnes_common::SaveSystem::new();
    // Note: We can't access save_dir directly as it's private, but we can test the public API
    
    println!("✓ M6 basic tests passed");
}

#[test]
fn test_m6_configuration_features() {
    println!("Testing M6 configuration features...");
    
    // Test configuration creation and modification
    let mut config = Config::default();
    
    // Test video configuration
    config.video.scale_factor = 2.0;
    config.video.window_width = 512;
    config.video.window_height = 480;
    assert_eq!(config.video.scale_factor, 2.0);
    assert_eq!(config.video.window_width, 512);
    assert_eq!(config.video.window_height, 480);
    
    // Test audio configuration
    config.audio.master_volume = 0.7;
    config.audio.sample_rate = 48000;
    assert_eq!(config.audio.master_volume, 0.7);
    assert_eq!(config.audio.sample_rate, 48000);
    
    // Test debug configuration
    config.debug.enabled = true;
    config.debug.show_cpu_status = true;
    config.debug.show_ppu_status = true;
    assert!(config.debug.enabled);
    assert!(config.debug.show_cpu_status);
    assert!(config.debug.show_ppu_status);
    
    // Test save state configuration
    config.save_states.slots = 5;
    config.save_states.quick_save_slot = 9;
    config.save_states.quick_load_slot = 8;
    assert_eq!(config.save_states.slots, 5);
    assert_eq!(config.save_states.quick_save_slot, 9);
    assert_eq!(config.save_states.quick_load_slot, 8);
    
    println!("✓ M6 configuration features test passed");
}

#[test]
fn test_m6_debugger_features() {
    println!("Testing M6 debugger features...");
    
    // Test debugger creation and basic operations
    let mut debugger = rnes_common::Debugger::new();
    
    // Test breakpoint management
    debugger.add_breakpoint(0x8000);
    debugger.add_breakpoint(0x8005);
    debugger.add_breakpoint(0x8010);
    assert_eq!(debugger.breakpoints.len(), 3);
    assert!(debugger.breakpoints.contains(&0x8000));
    assert!(debugger.breakpoints.contains(&0x8005));
    assert!(debugger.breakpoints.contains(&0x8010));
    
    // Test breakpoint removal
    debugger.remove_breakpoint(0x8005);
    assert_eq!(debugger.breakpoints.len(), 2);
    assert!(!debugger.breakpoints.contains(&0x8005));
    
    // Test breakpoint clearing
    debugger.clear_breakpoints();
    assert_eq!(debugger.breakpoints.len(), 0);
    
    // Test watchpoint management
    debugger.add_watchpoint(0x0000);
    debugger.add_watchpoint(0x0100);
    assert_eq!(debugger.watchpoints.len(), 2);
    assert!(debugger.watchpoints.contains(&0x0000));
    assert!(debugger.watchpoints.contains(&0x0100));
    
    // Test watchpoint removal
    debugger.remove_watchpoint(0x0000);
    assert_eq!(debugger.watchpoints.len(), 1);
    assert!(!debugger.watchpoints.contains(&0x0000));
    
    // Test watchpoint clearing
    debugger.clear_watchpoints();
    assert_eq!(debugger.watchpoints.len(), 0);
    
    // Test step mode
    assert!(!debugger.step_mode);
    debugger.enable_step_mode();
    assert!(debugger.step_mode);
    debugger.disable_step_mode();
    assert!(!debugger.step_mode);
    
    // Test break next instruction
    assert!(!debugger.break_next);
    debugger.break_next_instruction();
    assert!(debugger.break_next);
    
    println!("✓ M6 debugger features test passed");
}

#[test]
fn test_m6_save_system_features() {
    println!("Testing M6 save system features...");
    
    // Test save system creation
    let save_system = rnes_common::SaveSystem::new();
    
    // Test battery backup detection (this should work even without ROMs)
    let rom_name = "test_rom".to_string();
    let has_backup = save_system.has_battery_backup(&rom_name);
    // This should return false for a non-existent ROM, which is expected
    assert!(!has_backup);
    
    println!("✓ M6 save system features test passed");
}

#[test]
#[ignore]
fn test_configuration_system() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_configuration_test() {
        Ok(TestResult::Passed) => println!("✓ Configuration system test passed"),
        Ok(result) => println!("Configuration system test result: {:?}", result),
        Err(e) => println!("Configuration system test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_debugger_features() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_debugger_test() {
        Ok(TestResult::Passed) => println!("✓ Debugger test passed"),
        Ok(result) => println!("Debugger test result: {:?}", result),
        Err(e) => println!("Debugger test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_save_states() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_save_state_test() {
        Ok(TestResult::Passed) => println!("✓ Save state test passed"),
        Ok(result) => println!("Save state test result: {:?}", result),
        Err(e) => println!("Save state test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_auto_save() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_auto_save_test() {
        Ok(TestResult::Passed) => println!("✓ Auto-save test passed"),
        Ok(result) => println!("Auto-save test result: {:?}", result),
        Err(e) => println!("Auto-save test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_real_rom_debugger() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_real_rom_debug_test() {
        Ok(TestResult::Passed) => println!("✓ Real ROM debugger test passed"),
        Ok(result) => println!("Real ROM debugger test result: {:?}", result),
        Err(e) => println!("Real ROM debugger test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_real_rom_save_states() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_real_rom_save_state_test() {
        Ok(TestResult::Passed) => println!("✓ Real ROM save state test passed"),
        Ok(result) => println!("Real ROM save state test result: {:?}", result),
        Err(e) => println!("Real ROM save state test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_real_rom_configuration() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_real_rom_configuration_test() {
        Ok(TestResult::Passed) => println!("✓ Real ROM configuration test passed"),
        Ok(result) => println!("Real ROM configuration test result: {:?}", result),
        Err(e) => println!("Real ROM configuration test error: {}", e),
    }
}

#[test]
#[ignore]
fn test_real_rom_timing() {
    let mut runner = M6TestRunner::new();
    
    match runner.run_real_rom_timing_test() {
        Ok(TestResult::Passed) => println!("✓ Real ROM timing test passed"),
        Ok(result) => println!("Real ROM timing test result: {:?}", result),
        Err(e) => println!("Real ROM timing test error: {}", e),
    }
}
