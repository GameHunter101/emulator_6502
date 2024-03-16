use crate::{
    cpu::{Byte, Word, CPU},
    instructions::Instruction,
    memory::Memory,
};

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(
        cpu.status.interupt_disable,
        cpu_copy.status.interupt_disable
    );
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
}

// TAX
#[test]
fn tax_can_transfer_non_negative_non_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.a_register = 0x42;
    cpu.x_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTax as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0x42);
    assert_eq!(cpu.x_register, 0x42);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn tax_can_transfer_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.a_register = 0x0;
    cpu.x_register = 0x32;

    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTax as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0x0);
    assert_eq!(cpu.x_register, 0x0);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn tax_can_transfer_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.a_register = 0xFF;
    cpu.x_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsTax as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0xFF);
    assert_eq!(cpu.x_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// TAY
#[test]
fn tay_can_transfer_non_negative_non_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.a_register = 0x42;
    cpu.y_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTay as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0x42);
    assert_eq!(cpu.y_register, 0x42);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn tay_can_transfer_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.a_register = 0x0;
    cpu.y_register = 0x32;

    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTay as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0x0);
    assert_eq!(cpu.y_register, 0x0);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn tay_can_transfer_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.a_register = 0xFF;
    cpu.y_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsTay as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0xFF);
    assert_eq!(cpu.y_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// TXA
#[test]
fn txa_can_transfer_non_negative_non_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x42;
    cpu.a_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTxa as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x42);
    assert_eq!(cpu.a_register, 0x42);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn txa_can_transfer_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x0;
    cpu.a_register = 0x32;

    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTxa as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x0);
    assert_eq!(cpu.a_register, 0x0);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn txa_can_transfer_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0xFF;
    cpu.a_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsTxa as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0xFF);
    assert_eq!(cpu.a_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// TYA
#[test]
fn tya_can_transfer_non_negative_non_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0x42;
    cpu.a_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTya as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0x42);
    assert_eq!(cpu.a_register, 0x42);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn tya_can_transfer_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0x0;
    cpu.a_register = 0x32;

    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsTya as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0x0);
    assert_eq!(cpu.a_register, 0x0);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn tya_can_transfer_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0xFF;
    cpu.a_register = 0x32;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsTya as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0xFF);
    assert_eq!(cpu.a_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}
