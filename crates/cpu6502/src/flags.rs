use bitflags::bitflags;

bitflags! {
    /// CPU status flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct StatusFlags: u8 {
        /// Carry flag (C)
        const CARRY = 0x01;
        /// Zero flag (Z)
        const ZERO = 0x02;
        /// Interrupt disable flag (I)
        const INTERRUPT_DISABLE = 0x04;
        /// Decimal mode flag (D) - unused in 2A03
        const DECIMAL = 0x08;
        /// Break flag (B)
        const BREAK = 0x10;
        /// Unused flag (U)
        const UNUSED = 0x20;
        /// Overflow flag (V)
        const OVERFLOW = 0x40;
        /// Negative flag (N)
        const NEGATIVE = 0x80;
    }
}

impl StatusFlags {
    /// Set zero and negative flags
    pub fn set_zn(&mut self, value: u8) {
        self.set(StatusFlags::ZERO, value == 0);
        self.set(StatusFlags::NEGATIVE, value & 0x80 != 0);
    }
    
    /// Set carry, zero and negative flags
    pub fn set_czn(&mut self, value: u8, carry: bool) {
        self.set(StatusFlags::CARRY, carry);
        self.set_zn(value);
    }
    
    /// Set overflow flag
    pub fn set_overflow(&mut self, overflow: bool) {
        self.set(StatusFlags::OVERFLOW, overflow);
    }
    
    /// Get interrupt disable status
    pub fn interrupts_disabled(&self) -> bool {
        self.contains(StatusFlags::INTERRUPT_DISABLE)
    }
    
    /// Set interrupt disable status
    pub fn set_interrupt_disable(&mut self, disabled: bool) {
        self.set(StatusFlags::INTERRUPT_DISABLE, disabled);
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        StatusFlags::UNUSED | StatusFlags::INTERRUPT_DISABLE
    }
}

impl serde::Serialize for StatusFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for StatusFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u8::deserialize(deserializer)?;
        Ok(StatusFlags::from_bits(bits).unwrap_or_default())
    }
}
