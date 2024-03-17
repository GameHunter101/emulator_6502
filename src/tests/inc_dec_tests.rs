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

// INX
#[test]
fn inx_can_increment_a_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x0;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsInx as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x01);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn inx_can_increment_255() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0xFF;

    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsInx as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x0);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn inx_can_increment_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0xFE;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsInx as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// DEX
#[test]
fn dex_can_decrement_non_zero_non_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x0F;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsDex as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x0E);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dex_can_decrement_a_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x0;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsDex as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dex_can_decrement_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0xFF;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsDex as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0xFE);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// INY
#[test]
fn iny_can_increment_a_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0x0;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsIny as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0x01);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn iny_can_increment_255() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0xFF;

    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsIny as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0x0);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn iny_can_increment_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0xFE;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsIny as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// DEY
#[test]
fn dey_can_decrement_non_zero_non_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0x0F;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsDey as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0x0E);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dey_can_decrement_a_zero_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0x0;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsDey as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0xFF);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dey_can_decrement_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.y_register = 0xFF;

    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsDey as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.y_register, 0xFE);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// DEC
#[test]
fn dec_zero_page_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsDecZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0x57;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0x56);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_zero_page_x_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x10;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsDecZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0x57;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0x56);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_abs_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsDecAbs as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = 0x57;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x8000_u16], 0x56);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn dec_abs_x_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x10;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsDecAbsX as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8010] = 0x57;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0x8010_u16], 0x56);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// INC
#[test]
fn inc_zero_page_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsIncZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0x57;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0x58);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_zero_page_x_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x10;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsIncZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0x57;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0x58);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_abs_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsIncAbs as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = 0x57;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x8000_u16], 0x58);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn inc_abs_x_can_decrement_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.x_register = 0x10;

    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsIncAbsX as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8010] = 0x57;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0x8010_u16], 0x58);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);

    verify_unmodified_flags(&cpu, &cpu_copy);
}
