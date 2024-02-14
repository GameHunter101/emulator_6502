use crate::{
    cpu::{Byte, Instruction, InstructionsError, CPU},
    memory::Memory,
};

use super::load_tests::RegisterToTest;

static mut CPU: CPU = CPU::reset();
static mut MEMORY: Memory = Memory::initialize();

unsafe fn verify_unmodified_flags(cpu_copy: &CPU) {
    assert_eq!(CPU.carry, cpu_copy.carry);
    assert_eq!(CPU.interupt_disable, cpu_copy.interupt_disable);
    assert_eq!(CPU.decimal_mode, cpu_copy.decimal_mode);
    assert_eq!(CPU.break_command, cpu_copy.break_command);
    assert_eq!(CPU.overflow, cpu_copy.overflow);
}

fn test_store_zero_page(opcode: Instruction, register_to_test: RegisterToTest) {
    unsafe {
        CPU = CPU::reset();

        match register_to_test {
            RegisterToTest::A => CPU.a_register = 0x2F,
            RegisterToTest::X => CPU.x_register = 0x2F,
            RegisterToTest::Y => CPU.y_register = 0x2F,
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x80;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(3, &mut MEMORY);

        assert_eq!(cycles, Ok(3));

        assert_eq!(MEMORY[0x0080_u16], 0x002F);

        verify_unmodified_flags(&cpu_copy);
    }
}

fn test_store_zero_page_plus_register(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_to_add: RegisterToTest,
) {
    unsafe {
        CPU = CPU::reset();

        match register_to_test {
            RegisterToTest::A => CPU.a_register = 0x2F,
            RegisterToTest::X => CPU.x_register = 0x2F,
            RegisterToTest::Y => CPU.y_register = 0x2F,
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x80;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(cycles, Ok(4));

        match register_to_add {
            RegisterToTest::A => assert_eq!(MEMORY[0x80_u8 + CPU.a_register], 0x002F),
            RegisterToTest::X => assert_eq!(MEMORY[0x80_u8 + CPU.x_register], 0x002F),
            RegisterToTest::Y => assert_eq!(MEMORY[0x80_u8 + CPU.y_register], 0x002F),
        }

        verify_unmodified_flags(&cpu_copy);
    }
}

fn test_store_absolute(opcode: Instruction, register_to_test: RegisterToTest) {
    unsafe {
        CPU = CPU::reset();

        match register_to_test {
            RegisterToTest::A => CPU.a_register = 0x2F,
            RegisterToTest::X => CPU.x_register = 0x2F,
            RegisterToTest::Y => CPU.y_register = 0x2F,
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0xFFFE] = 0x44;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(cycles, Ok(4));

        assert_eq!(MEMORY[0x8044_u16], 0x002F);

        verify_unmodified_flags(&cpu_copy);
    }
}

// STA
#[test]
fn sta_zero_page_can_store_value() {
    test_store_zero_page(Instruction::InsStaZp, RegisterToTest::A);
}

#[test]
fn sta_zero_page_x_can_store_value() {
    test_store_zero_page_plus_register(
        Instruction::InsStaZpx,
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
    unsafe {
        CPU = CPU::reset();

        CPU.a_register = 0x2F;
        CPU.x_register = 0x04;

        MEMORY[0xFFFC] = Instruction::InsStaAbsX as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0xFFFE] = 0x44;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(5, &mut MEMORY);

        assert_eq!(cycles, Ok(5));

        assert_eq!(MEMORY[0x4484_u16], 0x002F);

        verify_unmodified_flags(&cpu_copy);
    }
}

#[test]
fn sta_absolute_y_can_store_value() {
    unsafe {
        CPU = CPU::reset();

        CPU.a_register = 0x2F;
        CPU.y_register = 0x04;

        MEMORY[0xFFFC] = Instruction::InsStaAbsY as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0xFFFE] = 0x44;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(5, &mut MEMORY);

        assert_eq!(cycles, Ok(5));

        assert_eq!(MEMORY[0x4484_u16], 0x002F);

        verify_unmodified_flags(&cpu_copy);
    }
}

#[test]
fn sta_indirect_x_can_store_value() {
    unsafe {
        CPU = CPU::reset();

        CPU.a_register = 0x2F;
        CPU.x_register = 0x04;

        MEMORY[0xFFFC] = Instruction::InsStaAbsY as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0x84] = 0x80;
        MEMORY[0x85] = 0x44;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(5, &mut MEMORY);

        assert_eq!(cycles, Ok(5));

        assert_eq!(MEMORY[0x4480_u16], 0x002F);

        verify_unmodified_flags(&cpu_copy);
    }
}

#[test]
fn sta_indirect_y_can_store_value() {
    unsafe {
        CPU = CPU::reset();

        CPU.a_register = 0x2F;
        CPU.y_register = 0x04;

        MEMORY[0xFFFC] = Instruction::InsStaAbsY as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0x80] = 0x80;
        MEMORY[0x81] = 0x44;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(5, &mut MEMORY);

        assert_eq!(cycles, Ok(5));

        assert_eq!(MEMORY[0x4484_u16], 0x002F);

        verify_unmodified_flags(&cpu_copy);
    }
}

// STX
#[test]
fn stx_zero_page_can_store_value() {
    test_store_zero_page(Instruction::InsStxZp, RegisterToTest::X);
}

#[test]
fn stx_zero_page_y_can_store_value() {
    test_store_zero_page_plus_register(
        Instruction::InsStxZpy,
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
        Instruction::InsStyZpx,
        RegisterToTest::Y,
        RegisterToTest::X,
    );
}

#[test]
fn sty_absolute_can_store_value() {
    test_store_absolute(Instruction::InsStyAbs, RegisterToTest::Y);
}
