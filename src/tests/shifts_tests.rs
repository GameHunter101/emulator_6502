use crate::{
    cpu::{Byte, SByte, Word, CPU},
    instructions::Instruction,
    memory::Memory,
};

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(
        cpu.status.interupt_disable,
        cpu_copy.status.interupt_disable
    );
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
}

// ASL

// Accumulator

#[test]
fn asl_a_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.a_register = 1;

    memory[0xFF00] = Instruction::InsAslA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 2);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn asl_a_can_shift_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.a_register = 0b11001010;

    memory[0xFF00] = Instruction::InsAslA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0b10010100);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Zero Page

#[test]
fn asl_zp_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsAslZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 2);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn asl_zp_can_shift_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsAslZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0b11001010;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0b10010100);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Zero Page X

#[test]
fn asl_zp_x_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsAslZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 2);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn asl_zp_x_can_shift_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsAslZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0b11001010;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0b10010100);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Absolute

#[test]
fn asl_abs_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsAslAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 2);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn asl_abs_can_shift_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsAslAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0b11001010;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0b10010100);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Absolute X

#[test]
fn asl_abs_x_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsAslAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 2);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn asl_abs_x_can_shift_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsAslAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0b11001010;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0b10010100);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// LSR

// Accumulator

#[test]
fn lsr_a_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.a_register = 1;

    memory[0xFF00] = Instruction::InsLsrA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn lsr_a_can_shift_zero_to_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.a_register = 2;

    memory[0xFF00] = Instruction::InsLsrA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

// Zero Page

#[test]
fn lsr_zp_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsLsrZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn lsr_zp_can_shift_zero_to_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsLsrZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 2;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

// Zero Page X

#[test]
fn lsr_zp_x_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsLsrZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn lsr_zp_x_can_shift_zero_to_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsLsrZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 2;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

// Absolute

#[test]
fn lsr_abs_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsLsrAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn lsr_abs_can_shift_zero_to_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsLsrAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 2;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

// Absolute X

#[test]
fn lsr_abs_x_can_shift_value_of_one() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsLsrAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn lsr_abs_x_can_shift_zero_to_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsLsrAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 2;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

// ROL

// Accumulator

#[test]
fn rol_a_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.a_register = 0;

    memory[0xFF00] = Instruction::InsRolA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_a_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.a_register = 0b10000000;

    memory[0xFF00] = Instruction::InsRolA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_a_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.a_register = 0;

    memory[0xFF00] = Instruction::InsRolA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_a_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.a_register = 0b01110011;

    memory[0xFF00] = Instruction::InsRolA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0b11100111);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Zero Page

#[test]
fn rol_zp_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRolZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_zp_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRolZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0b10000000;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_zp_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRolZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_zp_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsRolZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0b11100111);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Zero Page X

#[test]
fn rol_zp_x_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_zp_x_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0b10000000;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_zp_x_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_zp_x_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0b11100111);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Absolute

#[test]
fn rol_abs_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRolAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_abs_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRolAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0b10000000;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_abs_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRolAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_abs_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsRolAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0b11100111);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Absolute X

#[test]
fn rol_abs_x_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 1);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_abs_x_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0b10000000;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_abs_x_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn rol_abs_x_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRolAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0b11100111);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// ROR

// Accumulator

#[test]
fn ror_a_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.a_register = 0;

    memory[0xFF00] = Instruction::InsRorA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0b10000000);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

#[test]
fn ror_a_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.a_register = 1;

    memory[0xFF00] = Instruction::InsRorA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_a_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.a_register = 0;

    memory[0xFF00] = Instruction::InsRorA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_a_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.a_register = 0b01110011;

    memory[0xFF00] = Instruction::InsRorA as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, 0b10111001);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Zero Page

#[test]
fn ror_zp_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsRorZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0b10000000);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

#[test]
fn ror_zp_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRorZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_zp_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRorZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_zp_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsRorZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(memory[0x0042_u16], 0b10111001);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Zero Page X

#[test]
fn ror_zp_x_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0b10000000);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

#[test]
fn ror_zp_x_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_zp_x_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_zp_x_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0x0052_u16], 0b10111001);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Absolute

#[test]
fn ror_abs_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsRorAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0b10000000);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

#[test]
fn ror_abs_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRorAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_abs_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    memory[0xFF00] = Instruction::InsRorAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_abs_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    memory[0xFF00] = Instruction::InsRorAbs as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC542] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(memory[0xC542_u16], 0b10111001);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

// Absolute X

#[test]
fn ror_abs_x_can_shift_bit_out_of_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0b10000000);
    assert!(!cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

#[test]
fn ror_abs_x_can_shift_bit_into_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 1;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0);
    assert!(cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_abs_x_can_shift_zero_with_no_carry() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;
    cpu.status.zero = false;
    cpu.status.negative = true;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0);
    assert!(!cpu.status.carry);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn ror_abs_x_can_shift_value_resulting_in_negative_value() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;
    cpu.status.zero = true;
    cpu.status.negative = false;

    cpu.x_register = 0x10;

    memory[0xFF00] = Instruction::InsRorAbsX as Byte;
    memory[0xFF01] = 0x42;
    memory[0xFF02] = 0xC5;
    memory[0xC552] = 0b01110011;

    let cpu_copy = cpu;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[0xC552_u16], 0b10111001);
    assert!(cpu.status.carry);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}
