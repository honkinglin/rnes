use crate::{Byte, Word, RnesResult};

/// Memory access trait
pub trait MemoryAccess {
    /// Read byte
    fn read_byte(&self, addr: Word) -> RnesResult<Byte>;
    
    /// Write byte
    fn write_byte(&mut self, addr: Word, value: Byte) -> RnesResult<()>;
    
    /// Read word (little-endian)
    fn read_word(&self, addr: Word) -> RnesResult<Word> {
        let low = self.read_byte(addr)? as Word;
        let high = self.read_byte(addr + 1)? as Word;
        Ok(low | (high << 8))
    }
    
    /// Write word (little-endian)
    fn write_word(&mut self, addr: Word, value: Word) -> RnesResult<()> {
        self.write_byte(addr, value as Byte)?;
        self.write_byte(addr + 1, (value >> 8) as Byte)?;
        Ok(())
    }
}
