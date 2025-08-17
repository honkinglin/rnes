use rnes_common::{Byte, Word, AudioSample, RnesResult};
use std::collections::VecDeque;

// APU registers
const APU_PULSE1_VOLUME: u16 = 0x4000;
const APU_PULSE1_SWEEP: u16 = 0x4001;
const APU_PULSE1_FREQ_LOW: u16 = 0x4002;
const APU_PULSE1_FREQ_HIGH: u16 = 0x4003;

const APU_PULSE2_VOLUME: u16 = 0x4004;
const APU_PULSE2_SWEEP: u16 = 0x4005;
const APU_PULSE2_FREQ_LOW: u16 = 0x4006;
const APU_PULSE2_FREQ_HIGH: u16 = 0x4007;

const APU_TRIANGLE_LINEAR: u16 = 0x4008;
const APU_TRIANGLE_FREQ_LOW: u16 = 0x400A;
const APU_TRIANGLE_FREQ_HIGH: u16 = 0x400B;

const APU_NOISE_VOLUME: u16 = 0x400C;
const APU_NOISE_FREQ: u16 = 0x400E;
const APU_NOISE_LENGTH: u16 = 0x400F;

const APU_DMC_FREQ: u16 = 0x4010;
const APU_DMC_LOAD: u16 = 0x4011;
const APU_DMC_ADDR: u16 = 0x4012;
const APU_DMC_LENGTH: u16 = 0x4013;

const APU_STATUS: u16 = 0x4015;
const APU_FRAME_COUNTER: u16 = 0x4017;

// Audio constants
const SAMPLE_RATE: u32 = 44_100;
const CPU_CLOCK_RATE: u32 = 1_789_773;
const CYCLES_PER_SAMPLE: f32 = CPU_CLOCK_RATE as f32 / SAMPLE_RATE as f32;

// Pulse wave duty cycles
const PULSE_DUTY_CYCLES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [0, 0, 0, 0, 0, 0, 1, 1], // 25%
    [0, 0, 0, 0, 1, 1, 1, 1], // 50%
    [1, 1, 1, 1, 1, 1, 0, 0], // 75%
];

// Noise period table
const NOISE_PERIODS: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

// DMC period table
const DMC_PERIODS: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

// Length counter table
const LENGTH_COUNTER_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14,
    12, 16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30,
];

/// Pulse wave channel
#[derive(Debug, Clone)]
struct PulseChannel {
    enabled: bool,
    duty_cycle: u8,
    duty_step: u8,
    timer: u16,
    timer_value: u16,
    length_counter: u8,
    volume: u8,
    constant_volume: bool,
    envelope_start: bool,
    envelope_divider: u8,
    envelope_counter: u8,
    sweep_enabled: bool,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_negate: bool,
    sweep_reload: bool,
    sweep_counter: u8,
}

impl PulseChannel {
    fn new() -> Self {
        Self {
            enabled: false,
            duty_cycle: 0,
            duty_step: 0,
            timer: 0,
            timer_value: 0,
            length_counter: 0,
            volume: 0,
            constant_volume: false,
            envelope_start: false,
            envelope_divider: 0,
            envelope_counter: 0,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_negate: false,
            sweep_reload: false,
            sweep_counter: 0,
        }
    }

    fn step(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = self.timer_value;
            self.duty_step = (self.duty_step + 1) % 8;
        }
    }

    fn step_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_counter = 15;
            self.envelope_divider = 0;
        } else if self.envelope_divider > 0 {
            self.envelope_divider -= 1;
        } else {
            self.envelope_divider = self.volume;
            if self.envelope_counter > 0 {
                self.envelope_counter -= 1;
            } else if self.volume & 0x10 != 0 {
                self.envelope_counter = 15;
            }
        }
    }

    fn step_sweep(&mut self) {
        if self.sweep_reload {
            self.sweep_counter = self.sweep_period;
            self.sweep_reload = false;
        } else if self.sweep_counter > 0 {
            self.sweep_counter -= 1;
        } else {
            self.sweep_counter = self.sweep_period;
            if self.sweep_enabled && self.sweep_shift > 0 {
                let change = self.timer_value >> self.sweep_shift;
                if self.sweep_negate {
                    self.timer_value = self.timer_value.wrapping_sub(change);
                } else {
                    self.timer_value = self.timer_value.wrapping_add(change);
                }
            }
        }
    }

    fn step_length(&mut self) {
        if !self.constant_volume && self.length_counter > 0 {
            self.length_counter -= 1;
        }
    }

    fn output(&self) -> f32 {
        if !self.enabled || self.length_counter == 0 || self.timer_value < 8 {
            return 0.0;
        }

        let duty_value = PULSE_DUTY_CYCLES[self.duty_cycle as usize][self.duty_step as usize];
        if duty_value == 0 {
            return 0.0;
        }

        let envelope_output = if self.constant_volume {
            self.volume & 0x0F
        } else {
            self.envelope_counter
        };

        (envelope_output as f32) / 15.0
    }
}

/// Triangle wave channel
#[derive(Debug, Clone)]
struct TriangleChannel {
    enabled: bool,
    timer: u16,
    timer_value: u16,
    length_counter: u8,
    linear_counter: u8,
    linear_reload: u8,
    linear_reload_flag: bool,
    step: u8,
}

impl TriangleChannel {
    fn new() -> Self {
        Self {
            enabled: false,
            timer: 0,
            timer_value: 0,
            length_counter: 0,
            linear_counter: 0,
            linear_reload: 0,
            linear_reload_flag: false,
            step: 0,
        }
    }

    fn step(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = self.timer_value;
            if self.length_counter > 0 && self.linear_counter > 0 {
                self.step = (self.step + 1) % 32;
            }
        }
    }

    fn step_linear(&mut self) {
        if self.linear_reload_flag {
            self.linear_counter = self.linear_reload;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }
        if !self.linear_reload_flag {
            self.linear_reload_flag = false;
        }
    }

    fn step_length(&mut self) {
        if self.length_counter > 0 {
            self.length_counter -= 1;
        }
    }

    fn output(&self) -> f32 {
        if !self.enabled || self.length_counter == 0 || self.linear_counter == 0 {
            return 0.0;
        }

        // Triangle wave pattern
        let triangle_pattern = [
            15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ];

        (triangle_pattern[self.step as usize] as f32) / 15.0
    }
}

/// Noise channel
#[derive(Debug, Clone)]
struct NoiseChannel {
    enabled: bool,
    timer: u16,
    timer_value: u16,
    length_counter: u8,
    volume: u8,
    constant_volume: bool,
    envelope_start: bool,
    envelope_divider: u8,
    envelope_counter: u8,
    shift_register: u16,
    mode: bool,
}

impl NoiseChannel {
    fn new() -> Self {
        Self {
            enabled: false,
            timer: 0,
            timer_value: 0,
            length_counter: 0,
            volume: 0,
            constant_volume: false,
            envelope_start: false,
            envelope_divider: 0,
            envelope_counter: 0,
            shift_register: 1,
            mode: false,
        }
    }

    fn step(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = self.timer_value;
            
            let feedback = if self.mode {
                (self.shift_register >> 6) & 1
            } else {
                (self.shift_register >> 1) & 1
            } ^ (self.shift_register & 1);
            
            self.shift_register >>= 1;
            self.shift_register |= feedback << 14;
        }
    }

    fn step_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_counter = 15;
            self.envelope_divider = 0;
        } else if self.envelope_divider > 0 {
            self.envelope_divider -= 1;
        } else {
            self.envelope_divider = self.volume;
            if self.envelope_counter > 0 {
                self.envelope_counter -= 1;
            } else if self.volume & 0x10 != 0 {
                self.envelope_counter = 15;
            }
        }
    }

    fn step_length(&mut self) {
        if !self.constant_volume && self.length_counter > 0 {
            self.length_counter -= 1;
        }
    }

    fn output(&self) -> f32 {
        if !self.enabled || self.length_counter == 0 || (self.shift_register & 1) != 0 {
            return 0.0;
        }

        let envelope_output = if self.constant_volume {
            self.volume & 0x0F
        } else {
            self.envelope_counter
        };

        (envelope_output as f32) / 15.0
    }
}

/// DMC (Delta Modulation Channel)
#[derive(Debug, Clone)]
struct DmcChannel {
    enabled: bool,
    timer: u16,
    timer_value: u16,
    sample_buffer: u8,
    sample_buffer_empty: bool,
    shift_register: u8,
    bits_remaining: u8,
    sample_address: u16,
    sample_length: u16,
    current_address: u16,
    bytes_remaining: u16,
    loop_flag: bool,
    irq_enabled: bool,
    irq_pending: bool,
    output_level: u8,
}

impl DmcChannel {
    fn new() -> Self {
        Self {
            enabled: false,
            timer: 0,
            timer_value: 0,
            sample_buffer: 0,
            sample_buffer_empty: true,
            shift_register: 0,
            bits_remaining: 0,
            sample_address: 0,
            sample_length: 0,
            current_address: 0,
            bytes_remaining: 0,
            loop_flag: false,
            irq_enabled: false,
            irq_pending: false,
            output_level: 0,
        }
    }

    fn step(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = self.timer_value;
            
            if self.bits_remaining == 0 {
                if self.sample_buffer_empty {
                    // Start new sample fetch
                    if self.bytes_remaining > 0 {
                        // TODO: Implement memory reading
                        self.sample_buffer = 0;
                        self.sample_buffer_empty = false;
                        self.current_address = self.current_address.wrapping_add(1);
                        self.bytes_remaining -= 1;
                    } else if self.loop_flag {
                        // Restart sample
                        self.current_address = self.sample_address;
                        self.bytes_remaining = self.sample_length;
                    } else {
                        self.enabled = false;
                        if self.irq_enabled {
                            self.irq_pending = true;
                        }
                    }
                }
                
                if !self.sample_buffer_empty {
                    self.shift_register = self.sample_buffer;
                    self.bits_remaining = 8;
                    self.sample_buffer_empty = true;
                }
            }
            
            if self.bits_remaining > 0 {
                let bit = (self.shift_register >> 7) & 1;
                self.shift_register <<= 1;
                self.bits_remaining -= 1;
                
                if bit != 0 {
                    if self.output_level < 126 {
                        self.output_level += 2;
                    }
                } else {
                    if self.output_level > 1 {
                        self.output_level -= 2;
                    }
                }
            }
        }
    }

    fn output(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        (self.output_level as f32) / 127.0
    }
}

/// Frame counter for timing audio updates
#[derive(Debug, Clone)]
struct FrameCounter {
    mode: u8,
    step: u8,
    cycles: u32,
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            mode: 0,
            step: 0,
            cycles: 0,
        }
    }

    fn step(&mut self) -> bool {
        self.cycles += 1;
        
        let quarter_frame = match self.mode {
            0 => self.cycles % 7457 == 0,
            1 => self.cycles % 7457 == 0,
            _ => false,
        };
        
        let half_frame = match self.mode {
            0 => self.cycles % 14914 == 0,
            1 => self.cycles % 14914 == 0,
            _ => false,
        };
        
        if quarter_frame || half_frame {
            self.step = (self.step + 1) % 4;
            true
        } else {
            false
        }
    }
}

/// APU implementation
pub struct Apu {
    pulse1: PulseChannel,
    pulse2: PulseChannel,
    triangle: TriangleChannel,
    noise: NoiseChannel,
    dmc: DmcChannel,
    frame_counter: FrameCounter,
    sample_buffer: VecDeque<AudioSample>,
    cycles_since_sample: f32,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulse1: PulseChannel::new(),
            pulse2: PulseChannel::new(),
            triangle: TriangleChannel::new(),
            noise: NoiseChannel::new(),
            dmc: DmcChannel::new(),
            frame_counter: FrameCounter::new(),
            sample_buffer: VecDeque::new(),
            cycles_since_sample: 0.0,
        }
    }

    /// Step APU by one CPU cycle
    pub fn step(&mut self) -> RnesResult<()> {
        // Step frame counter
        if self.frame_counter.step() {
            // Quarter frame
            if self.frame_counter.step % 2 == 0 {
                self.pulse1.step_envelope();
                self.pulse2.step_envelope();
                self.noise.step_envelope();
                self.triangle.step_linear();
            }
            
            // Half frame
            if self.frame_counter.step % 2 == 1 {
                self.pulse1.step_sweep();
                self.pulse2.step_sweep();
                self.pulse1.step_length();
                self.pulse2.step_length();
                self.triangle.step_length();
                self.noise.step_length();
            }
        }

        // Step channels
        self.pulse1.step();
        self.pulse2.step();
        self.triangle.step();
        self.noise.step();
        self.dmc.step();

        // Generate audio samples
        self.cycles_since_sample += 1.0;
        if self.cycles_since_sample >= CYCLES_PER_SAMPLE {
            self.cycles_since_sample -= CYCLES_PER_SAMPLE;
            self.generate_sample();
        }

        Ok(())
    }

    /// Generate audio sample
    fn generate_sample(&mut self) {
        let pulse1_output = self.pulse1.output();
        let pulse2_output = self.pulse2.output();
        let triangle_output = self.triangle.output();
        let noise_output = self.noise.output();
        let dmc_output = self.dmc.output();

        // Mix audio channels
        let mixed = self.mix_audio(pulse1_output, pulse2_output, triangle_output, noise_output, dmc_output);
        
        self.sample_buffer.push_back(mixed);
        
        // Keep buffer size reasonable
        if self.sample_buffer.len() > 4096 {
            self.sample_buffer.pop_front();
        }
    }

    /// Mix audio channels
    fn mix_audio(&self, pulse1: f32, pulse2: f32, triangle: f32, noise: f32, dmc: f32) -> AudioSample {
        // NES audio mixing formula
        let pulse_out = 95.88 / (8128.0 / (pulse1 + pulse2) + 100.0);
        let tnd_out = 159.79 / (1.0 / (triangle / 8227.0 + noise / 12241.0 + dmc / 22638.0) + 100.0);
        
        (pulse_out + tnd_out) / 2.0
    }

    /// Read APU register
    pub fn read_register(&self, addr: Word) -> RnesResult<Byte> {
        match addr {
            APU_STATUS => {
                let mut status = 0;
                if self.pulse1.length_counter > 0 { status |= 0x01; }
                if self.pulse2.length_counter > 0 { status |= 0x02; }
                if self.triangle.length_counter > 0 { status |= 0x04; }
                if self.noise.length_counter > 0 { status |= 0x08; }
                if self.dmc.bytes_remaining > 0 { status |= 0x10; }
                if self.dmc.irq_pending { status |= 0x80; }
                Ok(status)
            }
            _ => Ok(0),
        }
    }

    /// Write APU register
    pub fn write_register(&mut self, addr: Word, value: Byte) -> RnesResult<()> {
        match addr {
            // Pulse 1
            APU_PULSE1_VOLUME => {
                self.pulse1.volume = value;
                self.pulse1.constant_volume = (value & 0x10) != 0;
                self.pulse1.duty_cycle = (value >> 6) & 0x03;
                self.pulse1.envelope_start = true;
            }
            APU_PULSE1_SWEEP => {
                self.pulse1.sweep_enabled = (value & 0x80) != 0;
                self.pulse1.sweep_period = (value >> 4) & 0x07;
                self.pulse1.sweep_negate = (value & 0x08) != 0;
                self.pulse1.sweep_shift = value & 0x07;
                self.pulse1.sweep_reload = true;
            }
            APU_PULSE1_FREQ_LOW => {
                self.pulse1.timer_value = (self.pulse1.timer_value & 0xFF00) | value as u16;
            }
            APU_PULSE1_FREQ_HIGH => {
                self.pulse1.timer_value = (self.pulse1.timer_value & 0x00FF) | (((value & 0x07) as u16) << 8);
                let length_index = (value >> 3) & 0x1F;
                self.pulse1.length_counter = LENGTH_COUNTER_TABLE[length_index as usize];
                self.pulse1.envelope_start = true;
            }

            // Pulse 2
            APU_PULSE2_VOLUME => {
                self.pulse2.volume = value;
                self.pulse2.constant_volume = (value & 0x10) != 0;
                self.pulse2.duty_cycle = (value >> 6) & 0x03;
                self.pulse2.envelope_start = true;
            }
            APU_PULSE2_SWEEP => {
                self.pulse2.sweep_enabled = (value & 0x80) != 0;
                self.pulse2.sweep_period = (value >> 4) & 0x07;
                self.pulse2.sweep_negate = (value & 0x08) != 0;
                self.pulse2.sweep_shift = value & 0x07;
                self.pulse2.sweep_reload = true;
            }
            APU_PULSE2_FREQ_LOW => {
                self.pulse2.timer_value = (self.pulse2.timer_value & 0xFF00) | value as u16;
            }
            APU_PULSE2_FREQ_HIGH => {
                self.pulse2.timer_value = (self.pulse2.timer_value & 0x00FF) | (((value & 0x07) as u16) << 8);
                let length_index = (value >> 3) & 0x1F;
                self.pulse2.length_counter = LENGTH_COUNTER_TABLE[length_index as usize];
                self.pulse2.envelope_start = true;
            }

            // Triangle
            APU_TRIANGLE_LINEAR => {
                self.triangle.linear_reload = value & 0x7F;
                self.triangle.linear_reload_flag = true;
            }
            APU_TRIANGLE_FREQ_LOW => {
                self.triangle.timer_value = (self.triangle.timer_value & 0xFF00) | value as u16;
            }
            APU_TRIANGLE_FREQ_HIGH => {
                self.triangle.timer_value = (self.triangle.timer_value & 0x00FF) | (((value & 0x07) as u16) << 8);
                let length_index = (value >> 3) & 0x1F;
                self.triangle.length_counter = LENGTH_COUNTER_TABLE[length_index as usize];
            }

            // Noise
            APU_NOISE_VOLUME => {
                self.noise.volume = value;
                self.noise.constant_volume = (value & 0x10) != 0;
                self.noise.envelope_start = true;
            }
            APU_NOISE_FREQ => {
                self.noise.mode = (value & 0x80) != 0;
                let period_index = (value & 0x0F) as usize;
                self.noise.timer_value = NOISE_PERIODS[period_index];
            }
            APU_NOISE_LENGTH => {
                let length_index = (value >> 3) & 0x1F;
                self.noise.length_counter = LENGTH_COUNTER_TABLE[length_index as usize];
                self.noise.envelope_start = true;
            }

            // DMC
            APU_DMC_FREQ => {
                self.dmc.irq_enabled = (value & 0x80) != 0;
                self.dmc.loop_flag = (value & 0x40) != 0;
                let period_index = (value & 0x0F) as usize;
                self.dmc.timer_value = DMC_PERIODS[period_index];
            }
            APU_DMC_LOAD => {
                self.dmc.output_level = value & 0x7F;
            }
            APU_DMC_ADDR => {
                self.dmc.sample_address = 0xC000 | ((value as u16) << 6);
            }
            APU_DMC_LENGTH => {
                self.dmc.sample_length = ((value as u16) << 4) | 1;
            }

            // Status
            APU_STATUS => {
                self.pulse1.enabled = (value & 0x01) != 0;
                self.pulse2.enabled = (value & 0x02) != 0;
                self.triangle.enabled = (value & 0x04) != 0;
                self.noise.enabled = (value & 0x08) != 0;
                self.dmc.enabled = (value & 0x10) != 0;
                
                if !self.pulse1.enabled { self.pulse1.length_counter = 0; }
                if !self.pulse2.enabled { self.pulse2.length_counter = 0; }
                if !self.triangle.enabled { self.triangle.length_counter = 0; }
                if !self.noise.enabled { self.noise.length_counter = 0; }
                if !self.dmc.enabled { self.dmc.bytes_remaining = 0; }
                
                self.dmc.irq_pending = false;
            }

            // Frame counter
            APU_FRAME_COUNTER => {
                self.frame_counter.mode = value & 0x80;
                // TODO: Handle 5-step sequence
            }

            _ => {
                tracing::debug!("Unknown APU register write: 0x{:04X} = 0x{:02X}", addr, value);
            }
        }

        Ok(())
    }

    /// Get audio samples
    pub fn get_samples(&mut self) -> Vec<AudioSample> {
        let samples: Vec<AudioSample> = self.sample_buffer.drain(..).collect();
        samples
    }

    /// Check if DMC IRQ is pending
    pub fn dmc_irq_pending(&self) -> bool {
        self.dmc.irq_pending
    }

    /// Clear DMC IRQ
    pub fn clear_dmc_irq(&mut self) {
        self.dmc.irq_pending = false;
    }
}

impl Default for Apu {
    fn default() -> Self {
        Self::new()
    }
}
