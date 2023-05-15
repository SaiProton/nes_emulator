use super::{Cpu, RegisterAlias};

pub fn load_data(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x55]);
    cpu.run();

    assert_eq!(*cpu.registers.by_alias(target), 0x55);
    assert!(!cpu.registers.status.get_flag('Z'));
    assert!(!cpu.registers.status.get_flag('N'));
}

pub fn flag_check(opcode: u8) {
    let mut cpu = Cpu::new();

    cpu.load_program(&[opcode, 0x00]);
    cpu.run();

    assert!(cpu.registers.status.get_flag('Z'));
    assert!(!cpu.registers.status.get_flag('N'));

    cpu.load_program(&[opcode, 0b1000_0000]);
    cpu.run();

    assert!(!cpu.registers.status.get_flag('Z'));
    assert!(cpu.registers.status.get_flag('N'));
}
