use rnes_core::Emulator;
use rnes_cartridge::Cartridge;
use rnes_common::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 RNES M6 Demo: Tools & Experience");
    println!("====================================");
    
    // Create a simple test ROM
    let rom_data = create_m6_test_rom();
    let cartridge = Cartridge::from_bytes(&rom_data)?;
    
    // Create emulator with custom configuration
    let mut config = Config::default();
    config.debug.enabled = true;
    config.debug.show_cpu_status = true;
    config.debug.show_ppu_status = true;
    config.general.auto_save_battery = true;
    config.general.auto_save_interval = 5; // 5 seconds
    config.save_states.quick_save_enabled = true;
    
    let mut emulator = Emulator::with_config(config);
    emulator.load_rom(cartridge)?;
    
    println!("✓ ROM loaded successfully");
    println!("✓ Configuration loaded");
    
    // Demo 1: Configuration System
    demo_configuration_system(&mut emulator)?;
    
    // Demo 2: Debugger Features
    demo_debugger_features(&mut emulator)?;
    
    // Demo 3: Save States
    demo_save_states(&mut emulator)?;
    
    // Demo 4: Auto-save
    demo_auto_save(&mut emulator)?;
    
    println!("\n🎉 M6 Demo completed successfully!");
    Ok(())
}

fn demo_configuration_system(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Demo 1: Configuration System");
    println!("-------------------------------");
    
    let config = emulator.get_config();
    println!("Window size: {}x{}", config.video.window_width, config.video.window_height);
    println!("Frame rate limit: {}", config.general.frame_rate_limit);
    println!("Audio enabled: {}", config.audio.enabled);
    println!("Debug mode: {}", config.debug.enabled);
    println!("Auto-save interval: {} seconds", config.general.auto_save_interval);
    
    // Modify configuration
    let config = emulator.get_config_mut();
    config.video.scale_factor = 4.0;
    config.audio.master_volume = 0.8;
    config.debug.show_memory_viewer = true;
    
    // Save configuration
    emulator.save_config()?;
    println!("✓ Configuration modified and saved");
    
    Ok(())
}

fn demo_debugger_features(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🐛 Demo 2: Debugger Features");
    println!("----------------------------");
    
    // Add breakpoints
    emulator.add_breakpoint(0x8005);
    emulator.add_breakpoint(0x800A);
    emulator.get_debugger_mut().add_watchpoint(0x0000);
    
    println!("✓ Added breakpoints at 0x8005 and 0x800A");
    println!("✓ Added watchpoint at 0x0000");
    
    // Enable step mode
    emulator.enable_step_mode();
    println!("✓ Step mode enabled");
    
    // Start emulator
    emulator.start();
    
    // Run a few steps
    println!("\nRunning emulator with debugger...");
    for step in 0..15 {
        match emulator.step() {
            Ok(_cycles) => {
                let debugger = emulator.get_debugger();
                let info = &debugger.debug_info;
                
                println!("Step {}: PC=0x{:04X}, A=0x{:02X}, X=0x{:02X}, Y=0x{:02X}", 
                        step, info.current_pc, info.cpu_registers.a, 
                        info.cpu_registers.x, info.cpu_registers.y);
                
                // Check if we hit a breakpoint
                if !emulator.is_running() {
                    println!("⏸️  Breakpoint hit at 0x{:04X}", info.current_pc);
                    emulator.start(); // Continue
                }
            }
            Err(e) => {
                println!("Step {}: ERROR - {}", step, e);
                break;
            }
        }
    }
    
    emulator.stop();
    emulator.disable_step_mode();
    
    // Show debugger info
    let debugger = emulator.get_debugger();
    println!("\nDebugger Statistics:");
    println!("  Breakpoints: {:?}", debugger.get_breakpoints());
    println!("  Watchpoints: {:?}", debugger.get_watchpoints());
    println!("  Instruction history: {} entries", debugger.instruction_history.len());
    println!("  Memory history: {} entries", debugger.memory_history.len());
    
    Ok(())
}

fn demo_save_states(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 Demo 3: Save States");
    println!("----------------------");
    
    // Run emulator to a specific state
    emulator.start();
    for _ in 0..50 {
        emulator.step()?;
    }
    emulator.stop();
    
    let initial_pc = emulator.cpu().pc;
    let initial_a = emulator.cpu().a;
    println!("Initial state: PC=0x{:04X}, A=0x{:02X}", initial_pc, initial_a);
    
    // Save state to slot 1
    emulator.save_state(1)?;
    println!("✓ Saved state to slot 1");
    
    // Continue running
    emulator.start();
    for _ in 0..30 {
        emulator.step()?;
    }
    emulator.stop();
    
    let new_pc = emulator.cpu().pc;
    let new_a = emulator.cpu().a;
    println!("New state: PC=0x{:04X}, A=0x{:02X}", new_pc, new_a);
    
    // Load state from slot 1
    emulator.load_state(1)?;
    println!("✓ Loaded state from slot 1");
    
    let restored_pc = emulator.cpu().pc;
    let restored_a = emulator.cpu().a;
    println!("Restored state: PC=0x{:04X}, A=0x{:02X}", restored_pc, restored_a);
    
    // Verify restoration
    if restored_pc == initial_pc && restored_a == initial_a {
        println!("✓ State restoration successful!");
    } else {
        println!("✗ State restoration failed!");
    }
    
    // Test quick save/load
    emulator.start();
    for _ in 0..20 {
        emulator.step()?;
    }
    emulator.stop();
    
    let quick_state_pc = emulator.cpu().pc;
    emulator.quick_save()?;
    println!("✓ Quick save completed");
    
    emulator.start();
    for _ in 0..10 {
        emulator.step()?;
    }
    emulator.stop();
    
    emulator.quick_load()?;
    println!("✓ Quick load completed");
    
    let quick_restored_pc = emulator.cpu().pc;
    if quick_restored_pc == quick_state_pc {
        println!("✓ Quick save/load successful!");
    } else {
        println!("✗ Quick save/load failed!");
    }
    
    Ok(())
}

fn demo_auto_save(emulator: &mut Emulator) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Demo 4: Auto-save");
    println!("-------------------");
    
    // Check if battery backup exists
    let rom_name = emulator.rom_name.as_ref().unwrap().clone();
    let has_backup = emulator.save_system.has_battery_backup(&rom_name);
    println!("Battery backup exists: {}", has_backup);
    
    // Run emulator to trigger auto-save
    emulator.start();
    println!("Running emulator for auto-save test...");
    
    // Run for a few seconds (simulated)
    for _ in 0..100 {
        emulator.step()?;
    }
    
    emulator.stop();
    
    // Manually trigger battery save
    emulator.save_battery_backup()?;
    println!("✓ Battery backup saved");
    
    // Check if battery backup now exists
    let has_backup_after = emulator.save_system.has_battery_backup(&rom_name);
    println!("Battery backup exists after save: {}", has_backup_after);
    
    Ok(())
}

fn create_m6_test_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    
    // iNES header
    rom.extend_from_slice(&[
        0x4E, 0x45, 0x53, 0x1A, // iNES magic
        0x01, 0x01,             // 16KB PRG, 8KB CHR
        0x00, 0x00,             // Mapper 0, horizontal mirroring
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
    ]);
    
    // PRG ROM (16KB) - Program that modifies registers and memory
    let mut prg_rom = vec![0; 16384];
    
    // Program: LDA #$42, STA $00, LDA #$84, STA $01, JMP $8000
    prg_rom[0] = 0xA9; // LDA immediate
    prg_rom[1] = 0x42; // Value 0x42
    prg_rom[2] = 0x85; // STA zero page
    prg_rom[3] = 0x00; // Address 0x00
    prg_rom[4] = 0xA9; // LDA immediate
    prg_rom[5] = 0x84; // Value 0x84
    prg_rom[6] = 0x85; // STA zero page
    prg_rom[7] = 0x01; // Address 0x01
    prg_rom[8] = 0x4C; // JMP absolute
    prg_rom[9] = 0x00; // Low byte
    prg_rom[10] = 0x80; // High byte (infinite loop)
    
    // Reset vector
    prg_rom[0x3FFC] = 0x00; // Reset vector low
    prg_rom[0x3FFD] = 0x80; // Reset vector high
    
    rom.extend_from_slice(&prg_rom);
    
    // CHR ROM (8KB) - Empty
    rom.extend(vec![0; 8192]);
    
    rom
}
