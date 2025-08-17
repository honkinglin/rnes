use serde::{Deserialize, Serialize};

/// 8-bit unsigned integer
pub type Byte = u8;

/// 16-bit unsigned integer
pub type Word = u16;

/// CPU cycle count
pub type Cycles = u32;

/// Scanline number
pub type Scanline = i32;

/// Dot position
pub type Dot = u32;

/// Pixel color (RGBA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub fn from_rgb(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: (rgb & 0xFF) as u8,
            a: 255,
        }
    }
    
    pub fn to_rgb(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    
    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
    
    pub fn from_u32(value: u32) -> Self {
        Self {
            r: ((value >> 24) & 0xFF) as u8,
            g: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
            a: (value & 0xFF) as u8,
        }
    }
}

/// Audio sample
pub type AudioSample = f32;

/// Controller button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

/// Controller state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerState {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Default for ControllerState {
    fn default() -> Self {
        Self {
            a: false,
            b: false,
            select: false,
            start: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

/// Emulator state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorState {
    pub cpu_cycles: Cycles,
    pub ppu_scanline: Scanline,
    pub ppu_dot: Dot,
    pub frame_count: u64,
}

impl Default for EmulatorState {
    fn default() -> Self {
        Self {
            cpu_cycles: 0,
            ppu_scanline: -1,
            ppu_dot: 0,
            frame_count: 0,
        }
    }
}
