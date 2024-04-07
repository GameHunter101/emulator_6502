use crate::{
    cpu::{Byte, CPU},
    instructions::{Instruction, InstructionsError},
    memory::Memory,
};

use super::load_tests::RegisterToTest;

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
}

fn test_store_zero_page(opcode: Instruction, register_to_test: RegisterToTest) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_to_test {
        RegisterToTest::A => cpu.a_register = 0x2F,
        RegisterToTest::X => cpu.x_register = 0x2F,
        RegisterToTest::Y => cpu.y_register = 0x2F,
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x80;

    let cpu_copy = cpu;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    assert_eq!(memory[0x0080_u16], 0x002F);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_store_zero_page_plus_register(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_to_add: RegisterToTest,
) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_to_test {
        RegisterToTest::A => cpu.a_register = 0x2F,
        RegisterToTest::X => cpu.x_register = 0x2F,
        RegisterToTest::Y => cpu.y_register = 0x2F,
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x80;

    let cpu_copy = cpu;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    match register_to_add {
        RegisterToTest::A => assert_eq!(memory[0x80_u8 + cpu.a_register], 0x002F),
        RegisterToTest::X => assert_eq!(memory[0x80_u8 + cpu.x_register], 0x002F),
        RegisterToTest::Y => assert_eq!(memory[0x80_u8 + cpu.y_register], 0x002F),
    }

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_store_absolute(opcode: Instruction, register_to_test: RegisterToTest) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_to_test {
        RegisterToTest::A => cpu.a_register = 0x2F,
        RegisterToTest::X => cpu.x_register = 0x2F,
        RegisterToTest::Y => cpu.y_register = 0x2F,
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x80;
    memory[0xFFFE] = 0x44;

    let cpu_copy = cpu;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    assert_eq!(memory[0x4480_u16], 0x002F);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// STA
#[test]
fn sta_zero_page_can_store_value() {
    test_store_zero_page(Instruction::InsStaZp, RegisterToTest::A);
}

#[test]
fn sta_zero_page_x_can_store_value() {
    test_store_zero_page_plus_register(
        Instruction::InsStaZpX,
        RegisterToTest::A,
        RegisterToTest::X,
    );
}

#[test]
fn sta_absolute_can_store_value() {
    test_store_absolute(Instruction::InsStaAbs, RegisterToTest::A);
}

#[test]
fn sta_absolute_x_can_store_value() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x2F;
    cpu.x_register = 0x04;

    memory[0xFFFC] = Instruction::InsStaAbsX as Byte;
    memory[0xFFFD] = 0x80;
    memory[0xFFFE] = 0x44;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    assert_eq!(memory[0x4484_u16], 0x002F);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn sta_absolute_y_can_store_value() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x2F;
    cpu.y_register = 0x04;

    memory[0xFFFC] = Instruction::InsStaAbsY as Byte;
    memory[0xFFFD] = 0x80;
    memory[0xFFFE] = 0x44;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    assert_eq!(memory[0x4484_u16], 0x002F);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn sta_indirect_x_can_store_value() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x2F;
    cpu.x_register = 0x04;

    memory[0xFFFC] = Instruction::InsStaIndX as Byte;
    memory[0xFFFD] = 0x80;
    memory[0x84] = 0x80;
    memory[0x85] = 0x44;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    assert_eq!(memory[0x4480_u16], 0x002F);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn sta_indirect_y_can_store_value() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x2F;
    cpu.y_register = 0x04;

    memory[0xFFFC] = Instruction::InsStaIndY as Byte;
    memory[0xFFFD] = 0x80;
    memory[0x80] = 0x80;
    memory[0x81] = 0x44;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    assert_eq!(memory[0x4484_u16], 0x002F);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// STX
#[test]
fn stx_zero_page_can_store_value() {
    test_store_zero_page(Instruction::InsStxZp, RegisterToTest::X);
}

#[test]
fn stx_zero_page_y_can_store_value() {
    test_store_zero_page_plus_register(
        Instruction::InsStxZpY,
        RegisterToTest::X,
        RegisterToTest::Y,
    );
}

#[test]
fn stx_absolute_can_store_value() {
    test_store_absolute(Instruction::InsStxAbs, RegisterToTest::X);
}

// STY
#[test]
fn sty_zero_page_can_store_value() {
    test_store_zero_page(Instruction::InsStyZp, RegisterToTest::Y);
}

#[test]
fn sty_zero_page_x_can_store_value() {
    test_store_zero_page_plus_register(
        Instruction::InsStyZpX,
        RegisterToTest::Y,
        RegisterToTest::X,
    );
}

#[test]
fn sty_absolute_can_store_value() {
    test_store_absolute(Instruction::InsStyAbs, RegisterToTest::Y);
}
