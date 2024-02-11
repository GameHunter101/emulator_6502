use crate::{
    cpu::{Byte, Instruction, InstructionsError, CPU},
    memory::Memory,
};

static mut CPU: CPU = CPU::reset();
static mut MEMORY: Memory = Memory::initialize();

enum RegisterToTest {
    A,
    X,
    Y,
}

unsafe fn verify_unmodified_flags_lda(cpu_copy: &CPU) {
    assert_eq!(CPU.carry, cpu_copy.carry);
    assert_eq!(CPU.interupt_disable, cpu_copy.interupt_disable);
    assert_eq!(CPU.decimal_mode, cpu_copy.decimal_mode);
    assert_eq!(CPU.break_command, cpu_copy.break_command);
    assert_eq!(CPU.overflow, cpu_copy.overflow);
}

#[test]
fn cpu_does_nothing_zero_cycles() {
    unsafe {
        let cycles = CPU.execute(0, &mut MEMORY);

        assert_eq!(cycles, Ok(0));
    }
}

#[test]
fn executing_bad_instruction_does_not_loop() {
    unsafe {
        CPU = CPU::reset();

        MEMORY[0xFFFC] = 0x0;
        MEMORY[0xFFFD] = 0x0;

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(
            cycles.is_err_and(|err| err == InstructionsError::InstructionDoesntExist(0x0)),
            true
        );
    }
}

fn test_loading_register_immediate(opcode: Instruction, register_to_test: RegisterToTest) {
    unsafe {
        CPU = CPU::reset();

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x84;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(2, &mut MEMORY);

        assert_eq!(cycles, Ok(2));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x84),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x84),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x84),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, true);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

fn test_loading_register_zero_page(opcode: Instruction, register_to_test: RegisterToTest) {
    unsafe {
        CPU = CPU::reset();

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x42;
        MEMORY[0x0042] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(3, &mut MEMORY);

        assert_eq!(cycles, Ok(3));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x37),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x37),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x37),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

fn test_loading_register_zero_page_plus_register(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    unsafe {
        CPU = CPU::reset();

        match register_being_added {
            RegisterToTest::X => CPU.x_register = 5,
            RegisterToTest::Y => CPU.y_register = 5,
            _ => {}
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x42;
        MEMORY[0x0047] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(cycles, Ok(4));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x37),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x37),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x37),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

fn test_loading_register_zero_page_plus_register_when_wrap(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    unsafe {
        CPU = CPU::reset();

        match register_being_added {
            RegisterToTest::X => CPU.x_register = 0xFF,
            RegisterToTest::Y => CPU.y_register = 0xFF,
            _ => {}
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0x007F] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(cycles, Ok(4));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x37),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x37),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x37),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

fn test_loading_register_absolute(opcode: Instruction, register_to_test: RegisterToTest) {
    unsafe {
        CPU = CPU::reset();

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0xFFFE] = 0x44;
        MEMORY[0x4480] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(cycles, Ok(4));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x37),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x37),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x37),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

fn test_loading_register_absolute_plus_register(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    unsafe {
        CPU = CPU::reset();

        match register_being_added {
            RegisterToTest::X => CPU.x_register = 1,
            RegisterToTest::Y => CPU.y_register = 1,
            _ => {}
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0xFFFE] = 0x44;
        MEMORY[0x4481] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(cycles, Ok(4));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x37),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x37),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x37),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

fn test_loading_register_absolute_plus_register_when_crossing_page_boundary(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    unsafe {
        CPU = CPU::reset();

        match register_being_added {
            RegisterToTest::X => CPU.x_register = 0xFF,
            RegisterToTest::Y => CPU.y_register = 0xFF,
            _ => {}
        }

        MEMORY[0xFFFC] = opcode as Byte;
        MEMORY[0xFFFD] = 0x02;
        MEMORY[0xFFFE] = 0x44;
        MEMORY[0x4501] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(5, &mut MEMORY);

        assert_eq!(cycles, Ok(5));

        match register_to_test {
            RegisterToTest::A => assert_eq!(CPU.a_register, 0x37),
            RegisterToTest::X => assert_eq!(CPU.x_register, 0x37),
            RegisterToTest::Y => assert_eq!(CPU.y_register, 0x37),
        }
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

#[test]
fn lda_immediate_can_load_value() {
    test_loading_register_immediate(Instruction::InsLdaIm, RegisterToTest::A);
}

#[test]
fn lda_immediate_can_affect_zero_flag() {
    unsafe {
        CPU = CPU::reset();

        CPU.a_register = 0x44;
        MEMORY[0xFFFC] = Instruction::InsLdaIm as Byte;
        MEMORY[0xFFFD] = 0x0;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(2, &mut MEMORY);

        assert_eq!(cycles, Ok(2));

        assert_eq!(CPU.a_register, 0x0);
        assert_eq!(CPU.zero, true);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

#[test]
fn lda_zero_page_can_load_value() {
    test_loading_register_zero_page(Instruction::InsLdaZp, RegisterToTest::A);
}

#[test]
fn lda_zero_page_x_can_load_value() {
    test_loading_register_zero_page_plus_register(
        Instruction::InsLdaZpx,
        RegisterToTest::A,
        RegisterToTest::X,
    );
}

#[test]
fn lda_zero_page_x_can_load_value_when_wrap() {
    test_loading_register_zero_page_plus_register_when_wrap(
        Instruction::InsLdaZpx,
        RegisterToTest::A,
        RegisterToTest::X,
    );
}

#[test]
fn lda_absolute_can_load_value() {
    test_loading_register_absolute(Instruction::InsLdaAbs, RegisterToTest::A);
}

#[test]
fn lda_absolute_x_can_load_value() {
    test_loading_register_absolute_plus_register(
        Instruction::InsLdaAbsX,
        RegisterToTest::A,
        RegisterToTest::X,
    );
}

#[test]
fn lda_absolute_x_can_load_value_when_crossing_page_boundary() {
    test_loading_register_absolute_plus_register_when_crossing_page_boundary(
        Instruction::InsLdaAbsX,
        RegisterToTest::A,
        RegisterToTest::X,
    );
}

#[test]
fn lda_absolute_y_can_load_value() {
    test_loading_register_absolute_plus_register(
        Instruction::InsLdaAbsY,
        RegisterToTest::A,
        RegisterToTest::Y,
    );
}

#[test]
fn lda_absolute_y_can_load_value_when_crossing_page_boundary() {
    test_loading_register_absolute_plus_register_when_crossing_page_boundary(
        Instruction::InsLdaAbsY,
        RegisterToTest::A,
        RegisterToTest::Y,
    );
}

#[test]
fn lda_indirect_x_can_load_value() {
    unsafe {
        CPU = CPU::reset();

        CPU.x_register = 0x04;

        MEMORY[0xFFFC] = Instruction::InsLdaIndX as Byte;
        MEMORY[0xFFFD] = 0x02;
        MEMORY[0x0006] = 0x00;
        MEMORY[0x0007] = 0x80;
        MEMORY[0x8000] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(6, &mut MEMORY);

        assert_eq!(cycles, Ok(6));

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

#[test]
fn lda_indirect_y_can_load_value() {
    unsafe {
        CPU = CPU::reset();

        CPU.y_register = 0x04;

        MEMORY[0xFFFC] = Instruction::InsLdaIndY as Byte;
        MEMORY[0xFFFD] = 0x02;
        MEMORY[0x0002] = 0x00;
        MEMORY[0x0003] = 0x80;
        MEMORY[0x8004] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(5, &mut MEMORY);

        assert_eq!(cycles, Ok(5));

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

#[test]
fn lda_indirect_y_can_load_value_when_crossing_page_boundary() {
    unsafe {
        CPU = CPU::reset();

        CPU.y_register = 0xFF;

        MEMORY[0xFFFC] = Instruction::InsLdaIndY as Byte;
        MEMORY[0xFFFD] = 0x02;
        MEMORY[0x0002] = 0x01;
        MEMORY[0x0003] = 0x80;
        MEMORY[0x8100] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(6, &mut MEMORY);

        assert_eq!(cycles, Ok(6));

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);
    }
}

// LDX

#[test]
fn ldx_immediate_can_load_value() {
    test_loading_register_immediate(Instruction::InsLdxIm, RegisterToTest::X);
}

#[test]
fn ldx_zero_page_can_load_value() {
    test_loading_register_zero_page(Instruction::InsLdxZp, RegisterToTest::X);
}

#[test]
fn ldx_zero_page_y_can_load_value() {
    test_loading_register_zero_page_plus_register(Instruction::InsLdxZpy, RegisterToTest::X, RegisterToTest::Y);
}

#[test]
fn ldx_zero_page_y_can_load_value_when_wrap() {
    test_loading_register_zero_page_plus_register_when_wrap(Instruction::InsLdxZpy, RegisterToTest::X, RegisterToTest::Y);
}

#[test]
fn ldx_absolute_can_load_value() {
    test_loading_register_absolute(Instruction::InsLdxAbs, RegisterToTest::X);
}

#[test]
fn ldx_absolute_y_can_load_value() {
    test_loading_register_absolute_plus_register(Instruction::InsLdxAbsY, RegisterToTest::X, RegisterToTest::Y);
}

#[test]
fn ldx_absolute_y_can_load_value_when_crossing_page_boundary() {
    test_loading_register_absolute_plus_register_when_crossing_page_boundary(Instruction::InsLdxAbsY, RegisterToTest::X, RegisterToTest::Y);
}

// LDY

#[test]
fn ldy_immediate_can_load_value() {
    test_loading_register_immediate(Instruction::InsLdyIm, RegisterToTest::Y);
}

#[test]
fn ldy_zero_page_can_load_value() {
    test_loading_register_zero_page(Instruction::InsLdyZp, RegisterToTest::Y);
}

#[test]
fn ldy_zero_page_x_can_load_value() {
    test_loading_register_zero_page_plus_register(Instruction::InsLdyZpx, RegisterToTest::Y, RegisterToTest::X);
}

#[test]
fn ldy_zero_page_x_can_load_value_when_wrap() {
    test_loading_register_zero_page_plus_register_when_wrap(Instruction::InsLdyZpx, RegisterToTest::Y, RegisterToTest::X);
}

#[test]
fn ldy_absolute_can_load_value() {
    test_loading_register_absolute(Instruction::InsLdyAbs, RegisterToTest::Y);
}

#[test]
fn ldy_absolute_x_can_load_value() {
    test_loading_register_absolute_plus_register(Instruction::InsLdyAbsX, RegisterToTest::Y, RegisterToTest::X);
}

#[test]
fn ldy_absolute_x_can_load_value_when_crossing_page_boundary() {
    test_loading_register_absolute_plus_register_when_crossing_page_boundary(Instruction::InsLdyAbsX, RegisterToTest::Y,RegisterToTest::X);
}
