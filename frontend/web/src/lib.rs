use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use std::sync::Arc;
use std::sync::Mutex;

use rnes_core::Emulator;
use rnes_common::{SCREEN_WIDTH, SCREEN_HEIGHT};
use rnes_cartridge::Cartridge;

/// Web NES Emulator
#[wasm_bindgen]
pub struct WebNesEmulator {
    emulator: Arc<Mutex<Emulator>>,
    canvas: HtmlCanvasElement,
    running: bool,
}

#[wasm_bindgen]
impl WebNesEmulator {
    /// Create new web emulator instance
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WebNesEmulator, JsValue> {
        console_error_panic_hook::set_once();
        
        // Get canvas element
        let window = web_sys::window().ok_or("No window found")?;
        let document = window.document().ok_or("No document found")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<HtmlCanvasElement>()?;
        
        // Set canvas size
        canvas.set_width(SCREEN_WIDTH as u32);
        canvas.set_height(SCREEN_HEIGHT as u32);
        
        let emulator = Arc::new(Mutex::new(Emulator::new()));
        
        Ok(WebNesEmulator {
            emulator,
            canvas,
            running: false,
        })
    }
    
    /// Load ROM from bytes
    pub fn load_rom_from_bytes(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        let cartridge = Cartridge::from_bytes(bytes)
            .map_err(|e| format!("Failed to load ROM: {}", e))?;
        self.load_rom(cartridge)
    }
    
    /// Start emulation
    pub fn start(&mut self) -> Result<(), JsValue> {
        if !self.running {
            self.running = true;
        }
        Ok(())
    }
    
    /// Stop emulation
    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.running = false;
        Ok(())
    }
    
    /// Reset emulator
    pub fn reset(&mut self) -> Result<(), JsValue> {
        let mut emu = self.emulator.lock().unwrap();
        emu.reset().map_err(|e| format!("Reset failed: {}", e))?;
        Ok(())
    }
    
    /// Step one CPU cycle
    pub fn step(&mut self) -> Result<u32, JsValue> {
        let mut emu = self.emulator.lock().unwrap();
        Ok(emu.step().map_err(|e| format!("Step failed: {}", e))?)
    }
    
    /// Get frame buffer as bytes
    pub fn get_frame_buffer(&self) -> Result<Vec<u8>, JsValue> {
        let emu = self.emulator.lock().unwrap();
        let frame_buffer = emu.bus.ppu().frame_buffer();
        
        let mut bytes = Vec::with_capacity(frame_buffer.len() * 4);
        for pixel in frame_buffer {
            bytes.push(pixel.r);
            bytes.push(pixel.g);
            bytes.push(pixel.b);
            bytes.push(255); // Alpha
        }
        
        Ok(bytes)
    }
    
    /// Check if emulator is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl WebNesEmulator {
    /// Load ROM into emulator
    fn load_rom(&mut self, cartridge: Cartridge) -> Result<(), JsValue> {
        let mut emu = self.emulator.lock().unwrap();
        emu.load_rom(cartridge)
            .map_err(|e| format!("Failed to load ROM: {}", e))?;
        Ok(())
    }
}

// Export utility functions
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn set_log_level(level: &str) {
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(level.parse().unwrap_or(tracing::Level::INFO))
            .build()
    );
}
