# M6 Implementation: Tools & Experience

## Overview

M6 milestone implements essential tools and user experience features for the NES emulator, including a comprehensive configuration system, debugging capabilities, and enhanced save state functionality.

## Components Implemented

### 1. Configuration System

**Location**: `crates/common/src/config.rs`

**Features**:
- TOML-based configuration file format
- Automatic configuration file creation and loading
- Comprehensive settings for all emulator components
- Cross-platform configuration directory support

**Key Components**:
- `Config` - Main configuration structure
- `GeneralConfig` - General emulator settings (vsync, frame rate, turbo mode)
- `VideoConfig` - Video settings (window size, scale factor, filters)
- `AudioConfig` - Audio settings (sample rate, volume, device)
- `InputConfig` - Input mappings and gamepad settings
- `DebugConfig` - Debug mode and debugging features
- `SaveStateConfig` - Save state slot management

**Configuration File Location**:
- macOS: `~/Library/Application Support/rnes/config.toml`
- Linux: `~/.config/rnes/config.toml`
- Windows: `%APPDATA%\rnes\config.toml`

**Example Configuration**:
```toml
[general]
vsync = true
frame_rate_limit = 60
turbo_mode = false
turbo_multiplier = 2.0
auto_save_battery = true
auto_save_interval = 30

[video]
window_width = 768
window_height = 720
fullscreen = false
scale_factor = 3.0
scanlines = false
scanline_intensity = 0.3
ntsc_filter = false
ntsc_strength = 0.5

[audio]
sample_rate = 44100
buffer_size = 1024
master_volume = 1.0
enabled = true
device_name = ""

[input]
enable_gamepad = true
gamepad_deadzone = 0.2

[input.controller1]
A = "Z"
B = "X"
Select = "Right Shift"
Start = "Enter"
Up = "Up"
Down = "Down"
Left = "Left"
Right = "Right"

[debug]
enabled = false
show_cpu_status = false
show_ppu_status = false
show_memory_viewer = false
show_disassembly = false
enable_breakpoints = false
step_execution = false
log_level = "info"

[save_states]
slots = 10
auto_save_slot = 0
quick_save_enabled = true
quick_save_slot = 9
quick_load_slot = 8
```

### 2. Debugger System

**Location**: `crates/common/src/debugger.rs`

**Features**:
- Breakpoint management (add/remove/clear)
- Watchpoint support for memory monitoring
- Step-by-step execution mode
- Comprehensive debug information collection
- Instruction and memory access history
- Real-time CPU and PPU state monitoring

**Key Components**:
- `Debugger` - Main debugger instance
- `DebugInfo` - Current execution state information
- `CpuRegisters` - CPU register state for debugging
- `StatusFlagsDebug` - CPU status flags breakdown
- `PpuDebugState` - PPU state for debugging
- `MemoryAccess` - Memory access tracking
- `InstructionInfo` - Instruction execution history

**Debugger API**:
```rust
// Breakpoint management
debugger.add_breakpoint(0x8000);
debugger.remove_breakpoint(0x8000);
debugger.clear_breakpoints();

// Watchpoint management
debugger.add_watchpoint(0x0000);
debugger.remove_watchpoint(0x0000);
debugger.clear_watchpoints();

// Step mode
debugger.enable_step_mode();
debugger.disable_step_mode();
debugger.break_next_instruction();

// History access
let recent_instructions = debugger.get_recent_instructions(10);
let recent_memory_accesses = debugger.get_recent_memory_accesses(10);
```

**Debug Information**:
- Current instruction address and bytes
- CPU register values (A, X, Y, SP, PC, Status)
- PPU state (scanline, dot, frame, vblank)
- Memory access patterns
- Instruction execution history

### 3. Enhanced Save State System

**Location**: `crates/common/src/save_system.rs` and `crates/core/src/emulator.rs`

**Features**:
- Multiple save state slots (configurable)
- Quick save/load functionality
- Auto-save for battery-backed games
- Comprehensive state serialization
- State validation and error handling

**Save State Components**:
- `SaveState` - Complete emulator state snapshot
- `CpuSaveState` - CPU register and status state
- `PpuSaveState` - PPU state including frame buffer
- `ApuSaveState` - Audio processing unit state
- `MemorySaveState` - RAM and PRG RAM state
- `MapperSaveState` - Mapper-specific state

**Save State API**:
```rust
// Save state to slot
emulator.save_state(1)?;

// Load state from slot
emulator.load_state(1)?;

// Check if save state exists
if emulator.has_save_state(1) {
    // Load existing state
}

// Quick save/load
emulator.quick_save()?;
emulator.quick_load()?;

// Delete save state
emulator.delete_save_state(1)?;
```

**Auto-save Features**:
- Configurable auto-save interval
- Battery backup auto-save
- Automatic save directory management
- Save state file validation

## Integration with Emulator Core

### Configuration Integration

The emulator core integrates the configuration system through:

```rust
// Create emulator with default configuration
let mut emulator = Emulator::new();

// Create emulator with custom configuration
let config = Config::default();
let mut emulator = Emulator::with_config(config);

// Access and modify configuration
let config = emulator.get_config();
let mut config = emulator.get_config_mut();
config.video.scale_factor = 4.0;

// Save configuration
emulator.save_config()?;
```

### Debugger Integration

The debugger is integrated into the emulator's execution loop:

```rust
// Check for breakpoints before each instruction
if self.debugger.should_break(self.cpu.pc) {
    self.running = false;
    self.debugger.break_next = false;
    return Ok(0);
}

// Update debug information
self.update_debug_info();

// Execute instruction
let cycles = self.bus.step_cpu(&mut self.cpu)?;
```

### Save State Integration

Save states are integrated throughout the emulator:

```rust
// Auto-save handling
fn handle_auto_save(&mut self) -> RnesResult<()> {
    if !self.config.general.auto_save_battery || 
       self.config.general.auto_save_interval == 0 {
        return Ok(());
    }
    
    let elapsed = self.last_auto_save.elapsed().as_secs() as u32;
    if elapsed >= self.config.general.auto_save_interval {
        self.save_battery_backup()?;
        self.last_auto_save = std::time::Instant::now();
    }
    
    Ok(())
}
```

## Testing Framework

### M6 Test Suite

**Location**: `crates/test-suite/src/m6_integration_tests.rs`

**Test Coverage**:
- Configuration system functionality
- Debugger breakpoint and watchpoint management
- Save state serialization and restoration
- Auto-save functionality
- Integration with emulator core

**Test Runner**:
```rust
pub struct M6TestRunner {
    emulator: Emulator,
    test_roms_dir: PathBuf,
}

impl M6TestRunner {
    pub fn run_configuration_test(&mut self) -> RnesResult<TestResult>;
    pub fn run_debugger_test(&mut self) -> RnesResult<TestResult>;
    pub fn run_save_state_test(&mut self) -> RnesResult<TestResult>;
    pub fn run_auto_save_test(&mut self) -> RnesResult<TestResult>;
}
```

### Demo Application

**Location**: `examples/m6_demo.rs`

**Features**:
- Interactive demonstration of all M6 features
- Configuration system showcase
- Debugger functionality demonstration
- Save state testing
- Auto-save functionality testing

## Usage Examples

### Basic Configuration

```rust
use rnes_core::Emulator;
use rnes_common::Config;

// Load or create configuration
let config = Config::load_or_create()?;

// Create emulator with configuration
let mut emulator = Emulator::with_config(config);

// Load ROM
emulator.load_rom(cartridge)?;
```

### Debugging Session

```rust
// Enable debug mode
emulator.get_config_mut().debug.enabled = true;

// Add breakpoints
emulator.add_breakpoint(0x8000);
emulator.add_breakpoint(0x8005);

// Enable step mode
emulator.enable_step_mode();

// Run with debugging
emulator.start();
while emulator.is_running() {
    emulator.step()?;
    
    // Check debug info
    let debugger = emulator.get_debugger();
    let info = &debugger.debug_info;
    println!("PC: 0x{:04X}, A: 0x{:02X}", info.current_pc, info.cpu_registers.a);
}
```

### Save State Management

```rust
// Save current state
emulator.save_state(1)?;

// Continue execution
for _ in 0..1000 {
    emulator.step()?;
}

// Restore previous state
emulator.load_state(1)?;

// Quick save/load
emulator.quick_save()?;
// ... later ...
emulator.quick_load()?;
```

## Performance Considerations

### Configuration System
- Lazy loading of configuration files
- Minimal memory overhead for configuration storage
- Efficient TOML parsing and serialization

### Debugger System
- Optional debug information collection
- Configurable history size limits
- Efficient breakpoint checking using HashSet

### Save State System
- Binary serialization for fast save/load
- Compressed save state format
- Incremental save state updates

## Future Enhancements

### Planned Features
- Visual debugger interface
- Memory viewer and editor
- Disassembly viewer
- Advanced breakpoint conditions
- Save state compression
- Configuration validation
- Hot-reload configuration changes

### Integration Opportunities
- Web frontend integration
- Plugin system for custom debuggers
- Network save state sharing
- Cloud save state synchronization

## Conclusion

M6 successfully implements a comprehensive set of tools and user experience features that significantly enhance the NES emulator's usability and debugging capabilities. The configuration system provides flexible customization options, the debugger offers powerful debugging tools, and the enhanced save state system ensures reliable game state management.

These features provide a solid foundation for advanced emulator functionality and user experience improvements in future milestones.
