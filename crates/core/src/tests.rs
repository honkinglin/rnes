use super::*;

#[test]
fn test_bus_creation() {
    let bus = Bus::new();
    assert_eq!(bus.ram[0], 0);
    assert!(bus.cartridge.is_none());
}

#[test]
fn test_bus_memory_access() {
    let mut bus = Bus::new();
    
    // Test RAM write and read
    bus.write_byte(0x0000, 0x42).unwrap();
    assert_eq!(bus.read_byte(0x0000).unwrap(), 0x42);
    
    // Test RAM mirroring
    bus.write_byte(0x2008, 0x84).unwrap();
    assert_eq!(bus.read_byte(0x2008).unwrap(), 0x84);
    assert_eq!(bus.read_byte(0x0008).unwrap(), 0x84); // Mirroring
    
    // Test word read/write
    bus.write_word(0x0001, 0x1234).unwrap();
    assert_eq!(bus.read_word(0x0001).unwrap(), 0x1234);
}

#[test]
fn test_emulator_creation() {
    let emulator = Emulator::new();
    assert!(!emulator.is_running());
    assert_eq!(emulator.get_state().cpu_cycles, 0);
}

#[test]
fn test_emulator_control() {
    let mut emulator = Emulator::new();
    
    emulator.start();
    assert!(emulator.is_running());
    
    emulator.stop();
    assert!(!emulator.is_running());
}
