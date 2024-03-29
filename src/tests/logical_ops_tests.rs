#![allow(clippy::clone_on_copy)]
use crate::{
    cpu::{Byte, CPU},
    instructions::{Instruction, InstructionsError},
    memory::Memory,
};

enum LogicalOperator {
    And,
    Eor,
    Ora,
}

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

fn do_logical_op(lhs: Byte, rhs: Byte, logical_operator: LogicalOperator) -> Byte {
    match logical_operator {
        LogicalOperator::And => lhs & rhs,
        LogicalOperator::Eor => lhs ^ rhs,
        LogicalOperator::Ora => lhs | rhs,
    }
}

fn test_logical_op_on_a_register_immediate(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndIm as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorIm as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraIm as Byte,
    }

    memory[0xFFFD] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_zero_page(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndZp as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorZp as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraZp as Byte,
    }

    memory[0xFFFD] = 0x4C;
    memory[0x004C] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_zero_page_x(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.x_register = 0x0F;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndZpX as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorZpX as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraZpX as Byte,
    }

    memory[0xFFFD] = 0x4C;
    memory[0x005B] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_absolute(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndAbs as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorAbs as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraAbs as Byte,
    }

    memory[0xFFFD] = 0x4C;
    memory[0xFFFE] = 0xAD;
    memory[0xAD4C] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_absolute_x(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(Some(0xFFF0));
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.x_register = 0x0F;

    match logical_operator {
        LogicalOperator::And => memory[0xFFF0] = Instruction::InsAndAbsX as Byte,
        LogicalOperator::Eor => memory[0xFFF0] = Instruction::InsEorAbsX as Byte,
        LogicalOperator::Ora => memory[0xFFF0] = Instruction::InsOraAbsX as Byte,
    }

    memory[0xFFF1] = 0x4C;
    memory[0xFFF2] = 0xAD;
    memory[0xAD5B] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_absolute_x_when_crossing_page_boundary(
    logical_operator: LogicalOperator,
) {
    let mut cpu = CPU::reset(Some(0xFFF0));
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.x_register = 0xFF;

    match logical_operator {
        LogicalOperator::And => memory[0xFFF0] = Instruction::InsAndAbsX as Byte,
        LogicalOperator::Eor => memory[0xFFF0] = Instruction::InsEorAbsX as Byte,
        LogicalOperator::Ora => memory[0xFFF0] = Instruction::InsOraAbsX as Byte,
    }

    memory[0xFFF1] = 0x4C;
    memory[0xFFF2] = 0xAD;
    memory[0xAE4B] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_absolute_y(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(Some(0xFFF0));
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.y_register = 0x0F;

    match logical_operator {
        LogicalOperator::And => memory[0xFFF0] = Instruction::InsAndAbsY as Byte,
        LogicalOperator::Eor => memory[0xFFF0] = Instruction::InsEorAbsY as Byte,
        LogicalOperator::Ora => memory[0xFFF0] = Instruction::InsOraAbsY as Byte,
    }

    memory[0xFFF1] = 0x4C;
    memory[0xFFF2] = 0xAD;
    memory[0xAD5B] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_absolute_y_when_crossing_page_boundary(
    logical_operator: LogicalOperator,
) {
    let mut cpu = CPU::reset(Some(0xFFF0));
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.y_register = 0xFF;

    match logical_operator {
        LogicalOperator::And => memory[0xFFF0] = Instruction::InsAndAbsY as Byte,
        LogicalOperator::Eor => memory[0xFFF0] = Instruction::InsEorAbsY as Byte,
        LogicalOperator::Ora => memory[0xFFF0] = Instruction::InsOraAbsY as Byte,
    }

    memory[0xFFF1] = 0x4C;
    memory[0xFFF2] = 0xAD;
    memory[0xAE4B] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_indirect_x(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.x_register = 0x3D;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndIndX as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorIndX as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraIndX as Byte,
    }

    memory[0xFFFD] = 0x4C;
    memory[0x89] = 0x03;
    memory[0x8A] = 0xFC;
    memory[0xFC03] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_indirect_x_wrapping_zero_page(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.x_register = 0xEC;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndIndX as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorIndX as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraIndX as Byte,
    }

    memory[0xFFFD] = 0x4C;
    memory[0x38] = 0x03;
    memory[0x39] = 0xFC;
    memory[0xFC03] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_indirect_x_split_by_zero_page(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.x_register = 0x01;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndIndX as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorIndX as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraIndX as Byte,
    }

    memory[0xFFFD] = 0xFE;
    memory[0xFF] = 0x03;
    memory[0x00] = 0xFC;
    memory[0xFC03] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_indirect_y(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.y_register = 0x4C;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndIndY as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorIndY as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraIndY as Byte,
    }

    memory[0xFFFD] = 0x3B;
    memory[0x3B] = 0x03;
    memory[0x3C] = 0xFC;
    memory[0xFC4F] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_logical_op_on_a_register_indirect_y_when_crossing_page_boundary(
    logical_operator: LogicalOperator,
) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.y_register = 0xFF;

    match logical_operator {
        LogicalOperator::And => memory[0xFFFC] = Instruction::InsAndIndY as Byte,
        LogicalOperator::Eor => memory[0xFFFC] = Instruction::InsEorIndY as Byte,
        LogicalOperator::Ora => memory[0xFFFC] = Instruction::InsOraIndY as Byte,
    }

    memory[0xFFFD] = 0x3B;
    memory[0x3B] = 0x03;
    memory[0x3C] = 0xFC;
    memory[0xFD02] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    let predicted_value = do_logical_op(0xCC, 0x84, logical_operator);
    assert_eq!(cpu.a_register, predicted_value);

    assert!(!cpu.status.zero);
    assert_eq!(
        cpu.status.negative,
        predicted_value & 0b10000000 == 0b10000000
    );

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// AND
#[test]
fn test_logical_op_and_on_a_register_immediate() {
    test_logical_op_on_a_register_immediate(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_zero_page() {
    test_logical_op_on_a_register_zero_page(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_zero_page_x() {
    test_logical_op_on_a_register_zero_page_x(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_absolute() {
    test_logical_op_on_a_register_absolute(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_absolute_x() {
    test_logical_op_on_a_register_absolute_x(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_absolute_x_when_crossing_page_boundary() {
    test_logical_op_on_a_register_absolute_x_when_crossing_page_boundary(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_absolute_y() {
    test_logical_op_on_a_register_absolute_y(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_absolute_y_when_crossing_page_boundary() {
    test_logical_op_on_a_register_absolute_y_when_crossing_page_boundary(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_indirect_x() {
    test_logical_op_on_a_register_indirect_x(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_indirect_x_wrapping_zero_page() {
    test_logical_op_on_a_register_indirect_x_wrapping_zero_page(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_indirect_x_split_by_zero_page() {
    test_logical_op_on_a_register_indirect_x_split_by_zero_page(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_indirect_y() {
    test_logical_op_on_a_register_indirect_y(LogicalOperator::And);
}

#[test]
fn test_logical_op_and_on_a_register_indirect_y_when_wrapping_page_boundary() {
    test_logical_op_on_a_register_indirect_y_when_crossing_page_boundary(LogicalOperator::And);
}

// EOR
#[test]
fn test_logical_op_eor_on_a_register_immediate() {
    test_logical_op_on_a_register_immediate(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_zero_page() {
    test_logical_op_on_a_register_zero_page(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_zero_page_x() {
    test_logical_op_on_a_register_zero_page_x(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute() {
    test_logical_op_on_a_register_absolute(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute_x() {
    test_logical_op_on_a_register_absolute_x(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute_x_when_crossing_page_boundary() {
    test_logical_op_on_a_register_absolute_x_when_crossing_page_boundary(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute_y() {
    test_logical_op_on_a_register_absolute_y(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_absolute_y_when_crossing_page_boundary() {
    test_logical_op_on_a_register_absolute_y_when_crossing_page_boundary(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_indirect_x() {
    test_logical_op_on_a_register_indirect_x(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_indirect_x_wrapping_zero_page() {
    test_logical_op_on_a_register_indirect_x_wrapping_zero_page(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_indirect_x_split_by_zero_page() {
    test_logical_op_on_a_register_indirect_x_split_by_zero_page(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_indirect_y() {
    test_logical_op_on_a_register_indirect_y(LogicalOperator::Eor);
}

#[test]
fn test_logical_op_eor_on_a_register_indirect_y_when_wrapping_page_boundary() {
    test_logical_op_on_a_register_indirect_y_when_crossing_page_boundary(LogicalOperator::Eor);
}

// ORA
#[test]
fn test_logical_op_ora_on_a_register_immediate() {
    test_logical_op_on_a_register_immediate(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_zero_page() {
    test_logical_op_on_a_register_zero_page(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_zero_page_x() {
    test_logical_op_on_a_register_zero_page_x(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_absolute() {
    test_logical_op_on_a_register_absolute(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_absolute_x() {
    test_logical_op_on_a_register_absolute_x(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_absolute_x_when_crossing_page_boundary() {
    test_logical_op_on_a_register_absolute_x_when_crossing_page_boundary(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_absolute_y() {
    test_logical_op_on_a_register_absolute_y(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_absolute_y_when_crossing_page_boundary() {
    test_logical_op_on_a_register_absolute_y_when_crossing_page_boundary(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_indirect_x() {
    test_logical_op_on_a_register_indirect_x(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_indirect_x_wrapping_zero_page() {
    test_logical_op_on_a_register_indirect_x_wrapping_zero_page(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_indirect_x_split_by_zero_page() {
    test_logical_op_on_a_register_indirect_x_split_by_zero_page(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_indirect_y() {
    test_logical_op_on_a_register_indirect_y(LogicalOperator::Ora);
}

#[test]
fn test_logical_op_ora_on_a_register_indirect_y_when_wrapping_page_boundary() {
    test_logical_op_on_a_register_indirect_y_when_crossing_page_boundary(LogicalOperator::Ora);
}

// BIT
#[test]
fn test_bit_zero_page() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.status.overflow = false;
    cpu.status.negative = false;

    memory[0xFFFC] = Instruction::InsBitZp as Byte;

    memory[0xFFFD] = 0x42;
    memory[0x0042] = 0xCC;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    assert_eq!(cpu.a_register, 0xCC);
    assert!(!cpu.status.zero);
    assert!(cpu.status.overflow);
    assert!(cpu.status.negative);
}

#[test]
fn test_bit_zero_page_modifies_zero_flag() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.status.overflow = true;
    cpu.status.negative = true;

    memory[0xFFFC] = Instruction::InsBitZp as Byte;

    memory[0xFFFD] = 0x42;
    memory[0x0042] = 0x33;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    assert_eq!(cpu.a_register, 0xCC);
    assert!(cpu.status.zero);
    assert!(!cpu.status.overflow);
    assert!(!cpu.status.negative);
}

#[test]
fn test_bit_zero_page_modifies_zero_negative_overflow_flag() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x33;
    cpu.status.overflow = false;
    cpu.status.negative = false;

    memory[0xFFFC] = Instruction::InsBitZp as Byte;

    memory[0xFFFD] = 0x42;
    memory[0x0042] = 0xCC;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    assert_eq!(cpu.a_register, 0x33);
    assert!(cpu.status.zero);
    assert!(cpu.status.overflow);
    assert!(cpu.status.negative);
}

#[test]
fn test_bit_absolute() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.status.overflow = false;
    cpu.status.negative = false;

    memory[0xFFFC] = Instruction::InsBitAbs as Byte;

    memory[0xFFFD] = 0x00;
    memory[0xFFFE] = 0x80;
    memory[0x8000] = 0xCC;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    assert_eq!(cpu.a_register, 0xCC);
    assert!(!cpu.status.zero);
    assert!(cpu.status.overflow);
    assert!(cpu.status.negative);
}

#[test]
fn test_bit_absolute_modifies_zero_flag() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;
    cpu.status.overflow = true;
    cpu.status.negative = true;

    memory[0xFFFC] = Instruction::InsBitAbs as Byte;

    memory[0xFFFD] = 0x00;
    memory[0xFFFE] = 0x80;
    memory[0x8000] = 0x33;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    assert_eq!(cpu.a_register, 0xCC);
    assert!(cpu.status.zero);
    assert!(!cpu.status.overflow);
    assert!(!cpu.status.negative);
}

#[test]
fn test_bit_absolute_modifies_zero_negative_overflow_flag() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x33;
    cpu.status.overflow = false;
    cpu.status.negative = false;

    memory[0xFFFC] = Instruction::InsBitAbs as Byte;

    memory[0xFFFD] = 0x00;
    memory[0xFFFE] = 0x80;
    memory[0x8000] = 0xCC;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    assert_eq!(cpu.a_register, 0x33);
    assert!(cpu.status.zero);
    assert!(cpu.status.overflow);
    assert!(cpu.status.negative);
}
