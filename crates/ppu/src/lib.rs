use rnes_common::{Byte, Word, RnesResult, Pixel, Scanline, Dot, SCREEN_WIDTH, SCREEN_HEIGHT, 
                  TOTAL_SCANLINES, DOTS_PER_SCANLINE, VISIBLE_SCANLINES, NES_PALETTE};
use rnes_mappers::Mapper;

/// Sprite data structure (4 bytes per sprite)
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub y: Byte,        // Y position (top-left corner)
    pub tile_id: Byte,  // Tile index
    pub attributes: Byte, // Attributes (palette, priority, flip)
    pub x: Byte,        // X position (top-left corner)
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            y: 0,
            tile_id: 0,
            attributes: 0,
            x: 0,
        }
    }
    
    /// Get sprite palette (0-3)
    pub fn palette(&self) -> Byte {
        self.attributes & 0x03
    }
    
    /// Check if sprite is behind background
    pub fn behind_background(&self) -> bool {
        self.attributes & 0x20 != 0
    }
    
    /// Check if sprite is flipped horizontally
    pub fn flip_horizontal(&self) -> bool {
        self.attributes & 0x40 != 0
    }
    
    /// Check if sprite is flipped vertically
    pub fn flip_vertical(&self) -> bool {
        self.attributes & 0x80 != 0
    }
    
    /// Get sprite height (8 or 16 pixels)
    pub fn height(&self, sprite_size: bool) -> Byte {
        if sprite_size { 16 } else { 8 }
    }
    
    /// Check if sprite is visible on current scanline
    pub fn is_visible_on_scanline(&self, scanline: usize, sprite_size: bool) -> bool {
        let height = self.height(sprite_size) as usize;
        let y_pos = self.y as usize;
        
        // Handle sprite wrapping around screen
        if y_pos >= 240 {
            return false;
        }
        
        // Check if scanline intersects with sprite
        scanline >= y_pos && scanline < y_pos + height
    }
}

/// Sprite rendering state
#[derive(Debug)]
pub struct SpriteRenderingState {
    pub sprites_on_scanline: Vec<Sprite>,
    pub sprite_pattern_data: Vec<[Byte; 8]>, // Pattern data for each sprite
    pub sprite_zero_on_scanline: bool,
    pub sprite_overflow: bool,
}

impl Default for SpriteRenderingState {
    fn default() -> Self {
        Self {
            sprites_on_scanline: Vec::new(),
            sprite_pattern_data: Vec::new(),
            sprite_zero_on_scanline: false,
            sprite_overflow: false,
        }
    }
}

/// PPU registers
#[derive(Debug, Clone, Copy)]
pub struct PpuRegisters {
    pub ppuctrl: Byte,    // 0x2000
    pub ppumask: Byte,    // 0x2001
    pub ppustatus: Byte,  // 0x2002
    pub oamaddr: Byte,    // 0x2003
    pub oamdata: Byte,    // 0x2004
    pub ppuscroll: Byte,  // 0x2005
    pub ppuaddr: Byte,    // 0x2006
    pub ppudata: Byte,    // 0x2007
}

impl Default for PpuRegisters {
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

/// PPU internal state
#[derive(Debug)]
pub struct PpuState {
    pub scanline: Scanline,
    pub dot: Dot,
    pub frame_count: u64,
    pub vblank: bool,
    pub sprite_overflow: bool,
    pub sprite_zero_hit: bool,
    
    // Internal registers
    pub v: Word,  // Current VRAM address
    pub t: Word,  // Temporary VRAM address
    pub x: Byte,  // Fine X scroll
    pub w: bool,  // Write toggle
    
    // Background rendering state
    pub bg_shift_high: Word,
    pub bg_shift_low: Word,
    pub bg_attr_shift_high: Word,
    pub bg_attr_shift_low: Word,
    pub bg_next_tile_id: Byte,
    pub bg_next_tile_attr: Byte,
    pub bg_next_tile_lsb: Byte,
    pub bg_next_tile_msb: Byte,
    
    // Sprite rendering state
    pub sprite_rendering: SpriteRenderingState,
    pub oam_dma_active: bool,
    pub oam_dma_cycles: u16,
    pub oam_dma_addr: Word,
}

impl Default for PpuState {
    fn default() -> Self {
        Self {
            scanline: -1,
            dot: 0,
            frame_count: 0,
            vblank: false,
            sprite_overflow: false,
            sprite_zero_hit: false,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            bg_shift_high: 0,
            bg_shift_low: 0,
            bg_attr_shift_high: 0,
            bg_attr_shift_low: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attr: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            sprite_rendering: SpriteRenderingState::default(),
            oam_dma_active: false,
            oam_dma_cycles: 0,
            oam_dma_addr: 0,
        }
    }
}

/// PPU implementation
pub struct Ppu {
    registers: PpuRegisters,
    state: PpuState,
    oam: [Byte; 256],           // Object Attribute Memory
    palette_ram: [Byte; 32],    // Palette RAM
    frame_buffer: Vec<Pixel>,   // Frame buffer
    mapper: Box<dyn Mapper>,
}

impl Ppu {
    pub fn new(mapper: Box<dyn Mapper>) -> Self {
        Self {
            registers: PpuRegisters::default(),
            state: PpuState::default(),
            oam: [0; 256],
            palette_ram: [0; 32],
            frame_buffer: vec![Pixel::BLACK; SCREEN_WIDTH * SCREEN_HEIGHT],
            mapper,
        }
    }
    
    /// Start OAM DMA transfer
    pub fn start_oam_dma(&mut self, page: Byte) {
        self.state.oam_dma_active = true;
        self.state.oam_dma_cycles = 0;
        self.state.oam_dma_addr = (page as Word) << 8;
    }
    
    /// Step OAM DMA transfer
    pub fn step_oam_dma(&mut self, memory: &dyn rnes_common::MemoryAccess) -> RnesResult<()> {
        if !self.state.oam_dma_active {
            return Ok(());
        }
        
        // OAM DMA takes 513 cycles (1 dummy read + 256 writes)
        if self.state.oam_dma_cycles < 513 {
            if self.state.oam_dma_cycles == 0 {
                // Dummy read cycle
                let _ = memory.read_byte(self.state.oam_dma_addr)?;
            } else {
                // Write to OAM
                let oam_addr = (self.state.oam_dma_cycles - 1) as usize;
                let data = memory.read_byte(self.state.oam_dma_addr)?;
                self.oam[oam_addr] = data;
                self.state.oam_dma_addr = self.state.oam_dma_addr.wrapping_add(1);
            }
            self.state.oam_dma_cycles += 1;
        } else {
            // DMA transfer complete
            self.state.oam_dma_active = false;
        }
        
        Ok(())
    }
    
    /// Check if OAM DMA is active
    pub fn oam_dma_active(&self) -> bool {
        self.state.oam_dma_active
    }
    
    /// Step PPU by one dot
    pub fn step(&mut self) -> RnesResult<()> {
        // Handle OAM DMA if active
        if self.state.oam_dma_active {
            // For now, we'll handle DMA in a simplified way
            // In a real implementation, this would need access to system memory
            return Ok(());
        }
        
        // Update scanline and dot
        self.state.dot += 1;
        if self.state.dot >= DOTS_PER_SCANLINE as Dot {
            self.state.dot = 0;
            self.state.scanline += 1;
            
            if self.state.scanline >= TOTAL_SCANLINES as Scanline {
                self.state.scanline = -1;
                self.state.frame_count += 1;
            }
        }
        
        // Handle different scanline phases
        if self.state.scanline < VISIBLE_SCANLINES as Scanline {
            // Visible scanlines
            self.render_visible_scanline()?;
        } else if self.state.scanline == VISIBLE_SCANLINES as Scanline {
            // Pre-render scanline
            self.render_pre_render_scanline()?;
        } else {
            // VBlank scanlines
            self.render_vblank_scanline()?;
        }
        
        // Step mapper for IRQ handling
        self.mapper.step();
        
        Ok(())
    }
    
    /// Render visible scanline
    fn render_visible_scanline(&mut self) -> RnesResult<()> {
        let scanline = self.state.scanline as usize;
        
        // Background rendering
        if self.is_background_enabled() {
            self.render_background_scanline(scanline)?;
        }
        
        // Sprite rendering
        if self.is_sprites_enabled() {
            self.render_sprite_scanline(scanline)?;
        }
        
        Ok(())
    }
    
    /// Render pre-render scanline
    fn render_pre_render_scanline(&mut self) -> RnesResult<()> {
        // Clear VBlank flag at dot 1
        if self.state.dot == 1 {
            self.state.vblank = false;
            self.registers.ppustatus &= !0x80;
        }
        
        // Background rendering (same as visible scanlines)
        if self.is_background_enabled() {
            self.render_background_scanline(0)?;
        }
        
        Ok(())
    }
    
    /// Render VBlank scanline
    fn render_vblank_scanline(&mut self) -> RnesResult<()> {
        // Set VBlank flag at scanline 241, dot 1
        if self.state.scanline == 241 && self.state.dot == 1 {
            self.state.vblank = true;
            self.registers.ppustatus |= 0x80;
        }
        
        Ok(())
    }
    
    /// Render background for a scanline
    fn render_background_scanline(&mut self, scanline: usize) -> RnesResult<()> {
        // Background rendering logic
        if self.state.dot >= 1 && self.state.dot <= 256 {
            let x = (self.state.dot - 1) as usize;
            
            // Get pixel color from background
            let color = self.get_background_pixel(x, scanline)?;
            
            // Write to frame buffer (only for visible scanlines)
            if scanline < SCREEN_HEIGHT {
                let pixel_index = scanline * SCREEN_WIDTH + x;
                if pixel_index < self.frame_buffer.len() {
                    self.frame_buffer[pixel_index] = color;
                }
            }
        }
        
        // Background tile fetching
        if self.state.dot >= 1 && self.state.dot <= 256 {
            self.fetch_background_tiles()?;
        }
        
        // Shift background shift registers
        if self.state.dot >= 1 && self.state.dot <= 256 {
            self.shift_background_registers();
        }
        
        Ok(())
    }
    
    /// Get background pixel color
    fn get_background_pixel(&self, x: usize, _scanline: usize) -> RnesResult<Pixel> {
        // Calculate fine X position
        let fine_x = (x + self.state.x as usize) % 8;
        
        // Get pixel data from shift registers
        let bit0 = (self.state.bg_shift_low >> (15 - fine_x)) & 1;
        let bit1 = (self.state.bg_shift_high >> (15 - fine_x)) & 1;
        let attr_bit0 = (self.state.bg_attr_shift_low >> (7 - fine_x)) & 1;
        let attr_bit1 = (self.state.bg_attr_shift_high >> (7 - fine_x)) & 1;
        
        let palette_index = (attr_bit1 << 1) | attr_bit0;
        let color_index = (bit1 << 1) | bit0;
        
        if color_index == 0 {
            // Background color (universal)
            Ok(Pixel::from_rgb(NES_PALETTE[0]))
        } else {
            // Get color from palette
            let palette_addr = 0x3F00 + (palette_index << 2) + color_index;
            let color_id = self.read_palette_ram(palette_addr)?;
            Ok(Pixel::from_rgb(NES_PALETTE[color_id as usize]))
        }
    }
    
    /// Fetch background tiles
    fn fetch_background_tiles(&mut self) -> RnesResult<()> {
        let cycle = self.state.dot as usize;
        
        match cycle {
            1..=256 => {
                // Tile fetching cycles
                match (cycle - 1) % 8 {
                    0 => {
                        // Fetch nametable byte
                        let addr = self.get_nametable_address()?;
                        self.state.bg_next_tile_id = self.read_vram(addr)?;
                    }
                    2 => {
                        // Fetch attribute byte
                        let addr = self.get_attribute_address()?;
                        let attr_byte = self.read_vram(addr)?;
                        let attr_shift = self.get_attribute_shift()?;
                        self.state.bg_next_tile_attr = (attr_byte >> attr_shift) & 0x03;
                    }
                    4 => {
                        // Fetch pattern table low byte
                        let addr = self.get_pattern_address(false)?;
                        self.state.bg_next_tile_lsb = self.read_vram(addr)?;
                    }
                    6 => {
                        // Fetch pattern table high byte
                        let addr = self.get_pattern_address(true)?;
                        self.state.bg_next_tile_msb = self.read_vram(addr)?;
                    }
                    7 => {
                        // Load shift registers
                        self.load_background_registers();
                    }
                    _ => {}
                }
                
                // Increment X scroll
                if cycle % 8 == 0 {
                    self.increment_scroll_x();
                }
            }
            257..=320 => {
                // Tile fetching for next scanline
                // (simplified implementation)
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Load background shift registers
    fn load_background_registers(&mut self) {
        // Shift existing data
        self.state.bg_shift_high = (self.state.bg_shift_high << 8) | (self.state.bg_next_tile_msb as Word);
        self.state.bg_shift_low = (self.state.bg_shift_low << 8) | (self.state.bg_next_tile_lsb as Word);
        
        // Load attribute data
        let attr_bit0 = (self.state.bg_next_tile_attr & 0x01) as Word;
        let attr_bit1 = (self.state.bg_next_tile_attr & 0x02) as Word >> 1;
        
        self.state.bg_attr_shift_high = (self.state.bg_attr_shift_high << 8) | (attr_bit1 << 7);
        self.state.bg_attr_shift_low = (self.state.bg_attr_shift_low << 8) | (attr_bit0 << 7);
    }
    
    /// Shift background registers
    fn shift_background_registers(&mut self) {
        // Shift registers are already shifted during loading
    }
    
    /// Get nametable address
    fn get_nametable_address(&self) -> RnesResult<Word> {
        let base = 0x2000 + (self.state.v & 0x0C00);
        let offset = self.state.v & 0x03FF;
        Ok(base | offset)
    }
    
    /// Get attribute address
    fn get_attribute_address(&self) -> RnesResult<Word> {
        let base = 0x23C0 + (self.state.v & 0x0C00);
        let offset = ((self.state.v >> 4) & 0x38) | ((self.state.v >> 2) & 0x07);
        Ok(base | offset)
    }
    
    /// Get attribute shift
    fn get_attribute_shift(&self) -> RnesResult<u8> {
        let coarse_x = (self.state.v & 0x001F) >> 1;
        let coarse_y = (self.state.v & 0x03E0) >> 6;
        Ok((((coarse_y & 0x02) << 1) | (coarse_x & 0x02)) as u8)
    }
    
    /// Get pattern address
    fn get_pattern_address(&self, high: bool) -> RnesResult<Word> {
        let pattern_table = if self.registers.ppuctrl & 0x10 != 0 { 0x1000 } else { 0x0000 };
        let tile_id = self.state.bg_next_tile_id as Word;
        let fine_y = (self.state.v & 0x7000) >> 12;
        let offset = if high { 8 } else { 0 };
        
        Ok(pattern_table + (tile_id << 4) + fine_y + offset)
    }
    
    /// Increment scroll X
    fn increment_scroll_x(&mut self) {
        if (self.state.v & 0x001F) == 31 {
            self.state.v &= !0x001F;
            self.state.v ^= 0x0400; // Toggle nametable
        } else {
            self.state.v += 1;
        }
    }
    
    /// Check if background is enabled
    fn is_background_enabled(&self) -> bool {
        self.registers.ppumask & 0x08 != 0
    }
    
    /// Check if sprites are enabled
    fn is_sprites_enabled(&self) -> bool {
        self.registers.ppumask & 0x10 != 0
    }
    
    /// Render sprite scanline
    fn render_sprite_scanline(&mut self, scanline: usize) -> RnesResult<()> {
        // Clear sprite rendering state for the new scanline
        self.state.sprite_rendering = SpriteRenderingState::default();
        
        // Evaluate sprites for this scanline
        self.evaluate_sprites(scanline)?;
        
        // Render sprites in order of Y position
        let sprites_to_render = self.state.sprite_rendering.sprites_on_scanline.clone();
        for sprite in sprites_to_render {
            self.render_sprite(sprite, scanline)?;
        }
        
        // Check for sprite zero hit
        if self.state.sprite_rendering.sprite_zero_on_scanline {
            self.state.sprite_zero_hit = true;
        }
        
        // Check for sprite overflow
        if self.state.sprite_rendering.sprite_overflow {
            self.state.sprite_overflow = true;
        }
        
        Ok(())
    }
    
    /// Evaluate sprites for current scanline
    fn evaluate_sprites(&mut self, scanline: usize) -> RnesResult<()> {
        let sprite_height = if self.registers.ppuctrl & 0x20 != 0 { 16 } else { 8 };
        
        // Scan OAM for sprites visible on this scanline
        for i in 0..64 { // 64 sprites in OAM
            let oam_addr = i * 4;
            let sprite_y = self.oam[oam_addr];
            let sprite_tile_id = self.oam[oam_addr + 1];
            let sprite_attributes = self.oam[oam_addr + 2];
            let sprite_x = self.oam[oam_addr + 3];
            
            // Check if sprite is visible on this scanline
            if sprite_y < 240 && sprite_y + sprite_height > scanline as Byte && sprite_y <= scanline as Byte {
                // Add sprite to scanline list
                let sprite = Sprite {
                    y: sprite_y,
                    tile_id: sprite_tile_id,
                    attributes: sprite_attributes,
                    x: sprite_x,
                };
                
                self.state.sprite_rendering.sprites_on_scanline.push(sprite);
                
                // Check for sprite zero hit
                if i == 0 {
                    self.state.sprite_rendering.sprite_zero_on_scanline = true;
                }
                
                // Check for sprite overflow (more than 8 sprites on scanline)
                if self.state.sprite_rendering.sprites_on_scanline.len() > 8 {
                    self.state.sprite_rendering.sprite_overflow = true;
                    break; // Only first 8 sprites are rendered
                }
            }
        }
        
        Ok(())
    }
    
    /// Render a single sprite
    fn render_sprite(&mut self, sprite: Sprite, scanline: usize) -> RnesResult<()> {
        let sprite_y_pos = sprite.y as usize;
        let sprite_height = if self.registers.ppuctrl & 0x20 != 0 { 16 } else { 8 };
        
        // Check if sprite is visible on this scanline
        if sprite_y_pos > scanline || sprite_y_pos + sprite_height as usize <= scanline {
            return Ok(());
        }
        
        let sprite_x_pos = sprite.x as usize;
        let sprite_y_offset = scanline - sprite_y_pos;
        
        // Get sprite pattern data
        let pattern_table = if self.registers.ppuctrl & 0x10 != 0 { 0x1000 } else { 0x0000 };
        let tile_addr = pattern_table + (sprite.tile_id as Word) * 16;
        
        // Read pattern data for the sprite row
        let pattern_low = self.read_vram(tile_addr + sprite_y_offset as Word)?;
        let pattern_high = self.read_vram(tile_addr + sprite_y_offset as Word + 8)?;
        
        // Render sprite pixels
        for pixel_x in 0..8 {
            let screen_x = sprite_x_pos + pixel_x;
            if screen_x >= SCREEN_WIDTH {
                continue;
            }
            
            // Get pixel data from pattern
            let pattern_x = if sprite.flip_horizontal() { 7 - pixel_x } else { pixel_x };
            let low_bit = (pattern_low >> (7 - pattern_x)) & 1;
            let high_bit = (pattern_high >> (7 - pattern_x)) & 1;
            let color_index = (high_bit << 1) | low_bit;
            
            // Skip transparent pixels
            if color_index == 0 {
                continue;
            }
            
            // Get sprite color
            let palette_base = 0x3F10 + (sprite.palette() as Word) * 4;
            let color_id = self.read_palette_ram(palette_base + color_index as Word)?;
            let color = Pixel::from_rgb(NES_PALETTE[color_id as usize]);
            
            // Write to frame buffer
            let pixel_index = scanline * SCREEN_WIDTH + screen_x;
            if pixel_index < self.frame_buffer.len() {
                // Check sprite-to-background priority
                let bg_pixel = self.frame_buffer[pixel_index];
                if !sprite.behind_background() || bg_pixel == Pixel::BLACK {
                    self.frame_buffer[pixel_index] = color;
                }
            }
        }
        
        Ok(())
    }
    
    /// Read from VRAM
    fn read_vram(&mut self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x0000..=0x1FFF => {
                // Pattern tables - detect A12 for MMC3 IRQ
                self.mapper.read_chr(addr)
            }
            0x2000..=0x3EFF => {
                // Nametables (with mirroring)
                let mirrored_addr = self.mirror_nametable_address(addr);
                self.mapper.read_chr(mirrored_addr)
            }
            0x3F00..=0x3F1F => {
                // Palette RAM
                let palette_addr = addr & 0x1F;
                Ok(self.palette_ram[palette_addr as usize])
            }
            0x3F20..=0x3FFF => {
                // Palette RAM mirror
                let palette_addr = addr & 0x1F;
                Ok(self.palette_ram[palette_addr as usize])
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Mirror nametable address
    fn mirror_nametable_address(&self, addr: Word) -> Word {
        let base = addr & 0x2C00;
        let offset = addr & 0x03FF;
        
        let mirrored_base = match self.mapper.mirroring() {
            rnes_cartridge::Mirroring::Horizontal => {
                if base == 0x2400 || base == 0x2C00 {
                    base - 0x0400
                } else {
                    base
                }
            }
            rnes_cartridge::Mirroring::Vertical => {
                if base == 0x2800 || base == 0x2C00 {
                    base - 0x0800
                } else {
                    base
                }
            }
            rnes_cartridge::Mirroring::SingleScreenA => 0x2000,
            rnes_cartridge::Mirroring::SingleScreenB => 0x2400,
            rnes_cartridge::Mirroring::FourScreen => base,
        };
        
        mirrored_base | offset
    }
    
    /// Read from palette RAM
    fn read_palette_ram(&self, addr: Word) -> RnesResult<Byte> {
        let index = (addr & 0x1F) as usize;
        if index < self.palette_ram.len() {
            Ok(self.palette_ram[index])
        } else {
            Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Write to palette RAM
    fn write_palette_ram(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        let index = (addr & 0x1F) as usize;
        if index < self.palette_ram.len() {
            self.palette_ram[index] = value;
            Ok(())
        } else {
            Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Get frame buffer
    pub fn frame_buffer(&self) -> &[Pixel] {
        &self.frame_buffer
    }
    
    /// Get OAM data
    pub fn oam(&self) -> &[Byte] {
        &self.oam
    }
    
    /// Get palette RAM data
    pub fn palette_ram(&self) -> &[Byte] {
        &self.palette_ram
    }
    
    /// Check if VBlank is active
    pub fn vblank(&self) -> bool {
        self.state.vblank
    }
    
    /// Get current scanline
    pub fn scanline(&self) -> Scanline {
        self.state.scanline
    }
    
    /// Get current dot
    pub fn dot(&self) -> Dot {
        self.state.dot
    }
    
    /// Get frame count
    pub fn frame_count(&self) -> u64 {
        self.state.frame_count
    }
    
    /// Debug: Get PPU register values
    pub fn debug_registers(&self) -> PpuRegisters {
        self.registers
    }
    
    /// Get PPU registers
    pub fn registers(&self) -> &PpuRegisters {
        &self.registers
    }
    
    /// Get PPU state
    pub fn state(&self) -> &PpuState {
        &self.state
    }
    
    /// Set scanline
    pub fn set_scanline(&mut self, scanline: Scanline) {
        self.state.scanline = scanline;
    }
    
    /// Set dot
    pub fn set_dot(&mut self, dot: Dot) {
        self.state.dot = dot;
    }
    
    /// Set frame count
    pub fn set_frame(&mut self, frame: u32) {
        self.state.frame_count = frame as u64;
    }
    
    /// Set VBlank state
    pub fn set_vblank(&mut self, vblank: bool) {
        self.state.vblank = vblank;
    }
    
    /// Set OAM data
    pub fn set_oam(&mut self, oam: Vec<u8>) {
        if oam.len() == self.oam.len() {
            self.oam.copy_from_slice(&oam);
        }
    }
    
    /// Set palette RAM data
    pub fn set_palette_ram(&mut self, palette_ram: Vec<u8>) {
        if palette_ram.len() == self.palette_ram.len() {
            self.palette_ram.copy_from_slice(&palette_ram);
        }
    }
    
    /// Set frame buffer
    pub fn set_frame_buffer(&mut self, frame_buffer: Vec<Pixel>) {
        if frame_buffer.len() == self.frame_buffer.len() {
            self.frame_buffer = frame_buffer;
        }
    }
    
    /// Get mapper instance
    pub fn mapper(&self) -> &dyn rnes_mappers::Mapper {
        self.mapper.as_ref()
    }
    
    /// Get mutable mapper instance
    pub fn mapper_mut(&mut self) -> &mut dyn rnes_mappers::Mapper {
        self.mapper.as_mut()
    }
    
    /// Debug: Get PPU internal state
    pub fn debug_state(&self) -> &PpuState {
        &self.state
    }
    
    /// Debug: Check if background is enabled
    pub fn debug_background_enabled(&self) -> bool {
        self.is_background_enabled()
    }
    
    /// Read PPU register
    pub fn read_register(&mut self, addr: Word) -> RnesResult<Byte> {
        match addr {
            0x2002 => {
                // PPUSTATUS
                let status = self.registers.ppustatus;
                self.registers.ppustatus &= !0x80; // Clear VBlank flag
                self.state.w = false; // Reset write toggle
                Ok(status)
            }
            0x2004 => {
                // OAMDATA
                let addr = self.registers.oamaddr as usize;
                Ok(self.oam[addr])
            }
            0x2007 => {
                // PPUDATA
                let value = self.read_vram(self.state.v)?;
                
                // Auto-increment address
                if self.registers.ppuctrl & 0x04 != 0 {
                    self.state.v += 32;
                } else {
                    self.state.v += 1;
                }
                
                Ok(value)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Write PPU register
    pub fn write_register(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x2000 => {
                // PPUCTRL
                self.registers.ppuctrl = value;
                self.state.t = (self.state.t & 0xF3FF) | ((value as Word & 0x03) << 10);
                Ok(())
            }
            0x2001 => {
                // PPUMASK
                self.registers.ppumask = value;
                Ok(())
            }
            0x2002 => {
                // PPUSTATUS is read-only, ignore writes
                Ok(())
            }
            0x2003 => {
                // OAMADDR
                self.registers.oamaddr = value;
                Ok(())
            }
            0x2004 => {
                // OAMDATA
                let addr = self.registers.oamaddr as usize;
                self.oam[addr] = value;
                self.registers.oamaddr = self.registers.oamaddr.wrapping_add(1);
                Ok(())
            }
            0x2005 => {
                // PPUSCROLL
                if !self.state.w {
                    // First write: X scroll
                    self.state.x = value & 0x07;
                    self.state.t = (self.state.t & 0xFFE0) | ((value as Word >> 3) & 0x1F);
                } else {
                    // Second write: Y scroll
                    self.state.t = (self.state.t & 0x8C1F) | 
                                  (((value as Word & 0x07) << 12) | 
                                   (((value as Word >> 3) & 0x1F) << 5));
                }
                self.state.w = !self.state.w;
                Ok(())
            }
            0x2006 => {
                // PPUADDR
                if !self.state.w {
                    // First write: high byte
                    self.state.t = (self.state.t & 0x00FF) | ((value as Word & 0x3F) << 8);
                } else {
                    // Second write: low byte
                    self.state.t = (self.state.t & 0xFF00) | value as Word;
                    self.state.v = self.state.t;
                }
                self.state.w = !self.state.w;
                Ok(())
            }
            0x2007 => {
                // PPUDATA
                self.write_vram(self.state.v, value)?;
                
                // Auto-increment address
                if self.registers.ppuctrl & 0x04 != 0 {
                    self.state.v += 32;
                } else {
                    self.state.v += 1;
                }
                Ok(())
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
    
    /// Write to VRAM
    fn write_vram(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            0x0000..=0x1FFF => {
                // Pattern tables
                self.mapper.write_chr(addr, value)
            }
            0x2000..=0x3EFF => {
                // Nametables (with mirroring)
                let mirrored_addr = self.mirror_nametable_address(addr);
                self.mapper.write_chr(mirrored_addr, value)
            }
            0x3F00..=0x3F1F => {
                // Palette RAM
                self.write_palette_ram(addr, value)
            }
            0x3F20..=0x3FFF => {
                // Palette RAM mirror
                self.write_palette_ram(addr, value)
            }
            _ => Err(rnes_common::RnesError::MemoryAccess { address: addr })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rnes_mappers::NromMapper;
    use rnes_cartridge::{Cartridge, Mirroring};
    
    #[test]
    fn test_sprite_creation() {
        let sprite = Sprite::new();
        assert_eq!(sprite.y, 0);
        assert_eq!(sprite.tile_id, 0);
        assert_eq!(sprite.attributes, 0);
        assert_eq!(sprite.x, 0);
    }
    
    #[test]
    fn test_sprite_attributes() {
        let mut sprite = Sprite::new();
        sprite.attributes = 0x23; // Palette 3, behind background, no flip
        
        assert_eq!(sprite.palette(), 3);
        assert!(sprite.behind_background());
        assert!(!sprite.flip_horizontal());
        assert!(!sprite.flip_vertical());
    }
    
    #[test]
    fn test_sprite_visibility() {
        let mut sprite = Sprite::new();
        sprite.y = 100;
        
        // 8x8 sprite
        assert!(sprite.is_visible_on_scanline(100, false));
        assert!(sprite.is_visible_on_scanline(107, false));
        assert!(!sprite.is_visible_on_scanline(108, false));
        
        // 16x16 sprite
        assert!(sprite.is_visible_on_scanline(100, true));
        assert!(sprite.is_visible_on_scanline(115, true));
        assert!(!sprite.is_visible_on_scanline(116, true));
    }
    
    #[test]
    fn test_ppu_creation_with_sprites() {
        // Create test ROM data
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
        let ppu = Ppu::new(Box::new(mapper));
        
        assert_eq!(ppu.oam_dma_active(), false);
        assert_eq!(ppu.state.sprite_rendering.sprites_on_scanline.len(), 0);
    }
    
    #[test]
    fn test_oam_dma_start() {
        // Create test ROM data
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
        
        ppu.start_oam_dma(0x02);
        assert!(ppu.oam_dma_active());
        assert_eq!(ppu.state.oam_dma_addr, 0x0200);
    }
}
