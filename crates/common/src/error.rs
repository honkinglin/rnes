use thiserror::Error;

/// RNES emulator error types
#[derive(Error, Debug)]
pub enum RnesError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("ROM format error: {0}")]
    RomFormat(String),
    
    #[error("Unsupported mapper type: {0}")]
    UnsupportedMapper(u8),
    
    #[error("Memory access error: address 0x{address:04X}")]
    MemoryAccess { address: u16 },
    
    #[error("CPU error: {0}")]
    Cpu(String),
    
    #[error("PPU error: {0}")]
    Ppu(String),
    
    #[error("APU error: {0}")]
    Apu(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Result type alias
pub type RnesResult<T> = Result<T, RnesError>;
