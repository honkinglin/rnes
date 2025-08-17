use super::*;

#[test]
fn test_cpu_creation() {
    let cpu = Cpu::new();
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.x, 0);
    assert_eq!(cpu.y, 0);
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu.status.bits(), StatusFlags::UNUSED.bits() | StatusFlags::INTERRUPT_DISABLE.bits());
}

#[test]
fn test_status_flags() {
    let mut flags = StatusFlags::default();
    
    // Test zero flag
    flags.set_zn(0);
    assert!(flags.contains(StatusFlags::ZERO));
    assert!(!flags.contains(StatusFlags::NEGATIVE));
    
    // Test negative flag
    flags.set_zn(0x80);
    assert!(!flags.contains(StatusFlags::ZERO));
    assert!(flags.contains(StatusFlags::NEGATIVE));
    
    // Test carry flag
    flags.set_czn(0xFF, true);
    assert!(flags.contains(StatusFlags::CARRY));
}

#[test]
fn test_addressing_modes() {
    // Need to simulate memory access here, skip for now
    // Will test through bus implementation later
}

#[test]
fn test_instruction_execution() {
    // Need to simulate memory access here, skip for now
    // Will test through bus implementation later
}
