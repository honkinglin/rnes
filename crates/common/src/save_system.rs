use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use crate::{Byte, RnesResult, RnesError};

/// Save system for NES emulator
/// Handles battery backup saves and save states
#[derive(Debug)]
pub struct SaveSystem {
    save_dir: PathBuf,
}

impl SaveSystem {
    /// Create new save system with default save directory
    pub fn new() -> Self {
        let save_dir = PathBuf::from("saves");
        Self { save_dir }
    }
    
    /// Create save system with custom save directory
    pub fn with_save_dir<P: AsRef<Path>>(save_dir: P) -> Self {
        Self {
            save_dir: save_dir.as_ref().to_path_buf(),
        }
    }
    
    /// Ensure save directory exists
    pub fn ensure_save_dir(&self) -> RnesResult<()> {
        if !self.save_dir.exists() {
            std::fs::create_dir_all(&self.save_dir)?;
        }
        Ok(())
    }
    
    /// Get battery backup save path for a ROM
    pub fn get_battery_save_path(&self, rom_name: &str) -> PathBuf {
        self.save_dir.join(format!("{}.sav", rom_name))
    }
    
    /// Get save state path for a ROM
    pub fn get_save_state_path(&self, rom_name: &str, slot: u8) -> PathBuf {
        self.save_dir.join(format!("{}.state{}", rom_name, slot))
    }
    
    /// Save battery backup data
    pub fn save_battery_backup(&self, rom_name: &str, data: &[Byte]) -> RnesResult<()> {
        self.ensure_save_dir()?;
        
        let save_path = self.get_battery_save_path(rom_name);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&save_path)?;
        
        file.write_all(data)?;
        tracing::info!("Battery backup saved to: {:?}", save_path);
        Ok(())
    }
    
    /// Load battery backup data
    pub fn load_battery_backup(&self, rom_name: &str) -> RnesResult<Vec<Byte>> {
        let save_path = self.get_battery_save_path(rom_name);
        
        if !save_path.exists() {
            return Ok(Vec::new());
        }
        
        let mut file = File::open(&save_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        
        tracing::info!("Battery backup loaded from: {:?}", save_path);
        Ok(data)
    }
    
    /// Check if battery backup exists
    pub fn has_battery_backup(&self, rom_name: &str) -> bool {
        self.get_battery_save_path(rom_name).exists()
    }
    
    /// Delete battery backup
    pub fn delete_battery_backup(&self, rom_name: &str) -> RnesResult<()> {
        let save_path = self.get_battery_save_path(rom_name);
        
        if save_path.exists() {
            std::fs::remove_file(&save_path)?;
            tracing::info!("Battery backup deleted: {:?}", save_path);
        }
        
        Ok(())
    }
}

/// Save state data structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveState {
    /// Version of the save state format
    pub version: u32,
    /// Timestamp when save state was created
    pub timestamp: u64,
    /// ROM name
    pub rom_name: String,
    /// CPU state
    pub cpu_state: CpuSaveState,
    /// PPU state
    pub ppu_state: PpuSaveState,
    /// APU state
    pub apu_state: ApuSaveState,
    /// Memory state
    pub memory_state: MemorySaveState,
    /// Mapper state
    pub mapper_state: MapperSaveState,
}

/// CPU save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpuSaveState {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
    pub cycles: u64,
}

/// PPU save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PpuSaveState {
    pub scanline: u16,
    pub dot: u16,
    pub frame: u32,
    pub vblank: bool,
    pub registers: PpuRegistersSaveState,
    pub oam: Vec<u8>, // OAM data
    pub palette_ram: Vec<u8>, // Palette RAM data
    pub frame_buffer: Vec<u32>, // Pixel data as u32
}

/// PPU registers save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PpuRegistersSaveState {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppuscroll: u8,
    pub ppuaddr: u8,
    pub ppudata: u8,
}

/// APU save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApuSaveState {
    pub frame_counter: u8,
    pub frame_counter_mode: u8,
    pub frame_counter_cycles: u64,
    pub channels: Vec<AudioChannelSaveState>,
}

/// Audio channel save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioChannelSaveState {
    pub channel_type: u8,
    pub enabled: bool,
    pub volume: u8,
    pub frequency: u16,
    pub duty_cycle: u8,
    pub envelope: EnvelopeSaveState,
    pub sweep: SweepSaveState,
    pub length_counter: u8,
}

/// Envelope save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvelopeSaveState {
    pub volume: u8,
    pub decay_level: u8,
    pub decay_counter: u8,
    pub r#loop: bool,
    pub constant_volume: bool,
}

/// Sweep save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SweepSaveState {
    pub enabled: bool,
    pub period: u8,
    pub shift: u8,
    pub negate: bool,
    pub reload: bool,
    pub counter: u8,
}

/// Memory save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemorySaveState {
    pub ram: Vec<u8>, // RAM data
    pub prg_ram: Vec<u8>,
}

/// Mapper save state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapperSaveState {
    pub mapper_type: u8,
    pub data: Vec<u8>, // Serialized mapper-specific data
}

impl SaveState {
    /// Create new save state
    pub fn new(rom_name: String) -> Self {
        Self {
            version: 1,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            rom_name,
            cpu_state: CpuSaveState::default(),
            ppu_state: PpuSaveState::default(),
            apu_state: ApuSaveState::default(),
            memory_state: MemorySaveState::default(),
            mapper_state: MapperSaveState::default(),
        }
    }
    
    /// Save state to file
    pub fn save_to_file(&self, save_system: &SaveSystem, slot: u8) -> RnesResult<()> {
        save_system.ensure_save_dir()?;
        
        let save_path = save_system.get_save_state_path(&self.rom_name, slot);
        let data = bincode::serialize(self)
            .map_err(|e| RnesError::Serialization(format!("Failed to serialize save state: {}", e)))?;
        
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&save_path)?;
        
        file.write_all(&data)?;
        tracing::info!("Save state saved to: {:?}", save_path);
        Ok(())
    }
    
    /// Load state from file
    pub fn load_from_file(save_system: &SaveSystem, rom_name: &str, slot: u8) -> RnesResult<Self> {
        let save_path = save_system.get_save_state_path(rom_name, slot);
        
        if !save_path.exists() {
            return Err(RnesError::Serialization("Save state file not found".to_string()));
        }
        
        let mut file = File::open(&save_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        
        let save_state: SaveState = bincode::deserialize(&data)
            .map_err(|e| RnesError::Serialization(format!("Failed to deserialize save state: {}", e)))?;
        
        tracing::info!("Save state loaded from: {:?}", save_path);
        Ok(save_state)
    }
    
    /// Check if save state exists
    pub fn exists(save_system: &SaveSystem, rom_name: &str, slot: u8) -> bool {
        save_system.get_save_state_path(rom_name, slot).exists()
    }
    
    /// Delete save state
    pub fn delete(save_system: &SaveSystem, rom_name: &str, slot: u8) -> RnesResult<()> {
        let save_path = save_system.get_save_state_path(rom_name, slot);
        
        if save_path.exists() {
            std::fs::remove_file(&save_path)?;
            tracing::info!("Save state deleted: {:?}", save_path);
        }
        
        Ok(())
    }
}

// Default implementations
impl Default for CpuSaveState {
    fn default() -> Self {
        Self {
            pc: 0,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            cycles: 0,
        }
    }
}

impl Default for PpuSaveState {
    fn default() -> Self {
        Self {
            scanline: 0,
            dot: 0,
            frame: 0,
            vblank: false,
            registers: PpuRegistersSaveState::default(),
            oam: vec![0; 256],
            palette_ram: vec![0; 32],
            frame_buffer: Vec::new(),
        }
    }
}

impl Default for PpuRegistersSaveState {
    fn default() -> Self {
        Self {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
        }
    }
}

impl Default for ApuSaveState {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            frame_counter_mode: 0,
            frame_counter_cycles: 0,
            channels: Vec::new(),
        }
    }
}

impl Default for MemorySaveState {
    fn default() -> Self {
        Self {
            ram: vec![0; 2048],
            prg_ram: Vec::new(),
        }
    }
}

impl Default for MapperSaveState {
    fn default() -> Self {
        Self {
            mapper_type: 0,
            data: Vec::new(),
        }
    }
}
