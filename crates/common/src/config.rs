use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::{RnesResult, RnesError, Button};

/// Emulator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// General emulator settings
    pub general: GeneralConfig,
    /// Video settings
    pub video: VideoConfig,
    /// Audio settings
    pub audio: AudioConfig,
    /// Input settings
    pub input: InputConfig,
    /// Debug settings
    pub debug: DebugConfig,
    /// Save state settings
    pub save_states: SaveStateConfig,
}

/// General emulator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Enable vsync
    pub vsync: bool,
    /// Frame rate limit (0 = unlimited)
    pub frame_rate_limit: u32,
    /// Enable turbo mode
    pub turbo_mode: bool,
    /// Turbo speed multiplier
    pub turbo_multiplier: f32,
    /// Auto-save battery backup
    pub auto_save_battery: bool,
    /// Auto-save interval (seconds, 0 = disabled)
    pub auto_save_interval: u32,
}

/// Video configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    /// Window width
    pub window_width: u32,
    /// Window height
    pub window_height: u32,
    /// Fullscreen mode
    pub fullscreen: bool,
    /// Scale factor
    pub scale_factor: f32,
    /// Enable scanlines
    pub scanlines: bool,
    /// Scanline intensity (0.0 - 1.0)
    pub scanline_intensity: f32,
    /// Enable NTSC filter
    pub ntsc_filter: bool,
    /// NTSC filter strength (0.0 - 1.0)
    pub ntsc_strength: f32,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Audio sample rate
    pub sample_rate: u32,
    /// Audio buffer size
    pub buffer_size: usize,
    /// Master volume (0.0 - 1.0)
    pub master_volume: f32,
    /// Enable audio
    pub enabled: bool,
    /// Audio device name (empty = default)
    pub device_name: String,
}

/// Input configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    /// Controller 1 key mappings
    pub controller1: ControllerConfig,
    /// Controller 2 key mappings
    pub controller2: ControllerConfig,
    /// Enable gamepad support
    pub enable_gamepad: bool,
    /// Gamepad deadzone (0.0 - 1.0)
    pub gamepad_deadzone: f32,
}

/// Controller configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerConfig {
    /// Button mappings
    pub buttons: HashMap<Button, String>,
}

/// Debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debug mode
    pub enabled: bool,
    /// Show CPU status
    pub show_cpu_status: bool,
    /// Show PPU status
    pub show_ppu_status: bool,
    /// Show memory viewer
    pub show_memory_viewer: bool,
    /// Show disassembly
    pub show_disassembly: bool,
    /// Enable breakpoints
    pub enable_breakpoints: bool,
    /// Enable step-by-step execution
    pub step_execution: bool,
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,
}

/// Save state configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveStateConfig {
    /// Number of save state slots
    pub slots: u8,
    /// Auto-save slot (0 = disabled)
    pub auto_save_slot: u8,
    /// Enable quick save/load
    pub quick_save_enabled: bool,
    /// Quick save slot
    pub quick_save_slot: u8,
    /// Quick load slot
    pub quick_load_slot: u8,
}

impl Config {
    /// Create default configuration
    pub fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            video: VideoConfig::default(),
            audio: AudioConfig::default(),
            input: InputConfig::default(),
            debug: DebugConfig::default(),
            save_states: SaveStateConfig::default(),
        }
    }
    
    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> RnesResult<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            let config = Self::default();
            config.save_to_file(path)?;
            return Ok(config);
        }
        
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let config: Config = toml::from_str(&contents)
            .map_err(|e| RnesError::Serialization(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> RnesResult<()> {
        let path = path.as_ref();
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        
        let contents = toml::to_string_pretty(self)
            .map_err(|e| RnesError::Serialization(format!("Failed to serialize config: {}", e)))?;
        
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        
        file.write_all(contents.as_bytes())?;
        tracing::info!("Configuration saved to: {:?}", path);
        Ok(())
    }
    
    /// Get configuration file path
    pub fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("rnes");
        path.push("config.toml");
        path
    }
    
    /// Load or create default configuration
    pub fn load_or_create() -> RnesResult<Self> {
        let config_path = Self::get_config_path();
        Self::load_from_file(config_path)
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            vsync: true,
            frame_rate_limit: 60,
            turbo_mode: false,
            turbo_multiplier: 2.0,
            auto_save_battery: true,
            auto_save_interval: 30,
        }
    }
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            window_width: 256 * 3,
            window_height: 240 * 3,
            fullscreen: false,
            scale_factor: 3.0,
            scanlines: false,
            scanline_intensity: 0.3,
            ntsc_filter: false,
            ntsc_strength: 0.5,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            buffer_size: 1024,
            master_volume: 1.0,
            enabled: true,
            device_name: String::new(),
        }
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        let mut controller1 = HashMap::new();
        controller1.insert(Button::A, "Z".to_string());
        controller1.insert(Button::B, "X".to_string());
        controller1.insert(Button::Select, "Right Shift".to_string());
        controller1.insert(Button::Start, "Enter".to_string());
        controller1.insert(Button::Up, "Up".to_string());
        controller1.insert(Button::Down, "Down".to_string());
        controller1.insert(Button::Left, "Left".to_string());
        controller1.insert(Button::Right, "Right".to_string());
        
        let mut controller2 = HashMap::new();
        controller2.insert(Button::A, "K".to_string());
        controller2.insert(Button::B, "L".to_string());
        controller2.insert(Button::Select, "O".to_string());
        controller2.insert(Button::Start, "P".to_string());
        controller2.insert(Button::Up, "W".to_string());
        controller2.insert(Button::Down, "S".to_string());
        controller2.insert(Button::Left, "A".to_string());
        controller2.insert(Button::Right, "D".to_string());
        
        Self {
            controller1: ControllerConfig { buttons: controller1 },
            controller2: ControllerConfig { buttons: controller2 },
            enable_gamepad: true,
            gamepad_deadzone: 0.2,
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            show_cpu_status: false,
            show_ppu_status: false,
            show_memory_viewer: false,
            show_disassembly: false,
            enable_breakpoints: false,
            step_execution: false,
            log_level: "info".to_string(),
        }
    }
}

impl Default for SaveStateConfig {
    fn default() -> Self {
        Self {
            slots: 10,
            auto_save_slot: 0,
            quick_save_enabled: true,
            quick_save_slot: 9,
            quick_load_slot: 8,
        }
    }
}
