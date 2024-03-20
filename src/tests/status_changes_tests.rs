use crate::{
    cpu::{Byte, Word, CPU, SByte},
    instructions::Instruction,
    memory::Memory,
};

// CLC
#[test]
fn clc_will_clear_carry_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsClc as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(!cpu.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}

// SEC
#[test]
fn sec_will_set_carry_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsSec as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(cpu.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}

// CLD
#[test]
fn cld_will_clear_decimal_mode_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.decimal_mode = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCld as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(!cpu.status.decimal_mode);
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}

// SED
#[test]
fn sed_will_set_decimal_mode_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsSed as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(cpu.status.decimal_mode);
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}

// CLI
#[test]
fn cli_will_clear_interrupt_disable_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.decimal_mode = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCli as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(!cpu.status.interupt_disable);
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}

// SEI
#[test]
fn sei_will_set_interrupt_disable_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.interupt_disable = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsSei as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(cpu.status.interupt_disable);
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}

// CLV
#[test]
fn clv_will_clear_overflow_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.overflow = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsClv as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert!(!cpu.status.overflow);
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.zero, cpu_copy.status.zero);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.negative, cpu_copy.status.negative);
}
