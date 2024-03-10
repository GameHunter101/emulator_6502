use crate::{
    cpu::{Byte, CPU},
    instructions::{Instruction, InstructionsError},
    memory::Memory,
};

pub enum RegisterToTest {
    A,
    X,
    Y,
}

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(cpu.status.interupt_disable, cpu_copy.status.interupt_disable);
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
}

#[test]
fn cpu_does_nothing_zero_cycles() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();
    let cycles = cpu.execute(0, &mut memory);

    assert_eq!(cycles, Ok(0));
}

#[test]
fn executing_bad_instruction_does_not_loop() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    memory[0xFFFC] = 0x0;
    memory[0xFFFD] = 0x0;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(
        cycles.is_err_and(|err| err == InstructionsError::InstructionDoesntExist(0x0)),
        true
    );
}

fn test_loading_register_immediate(opcode: Instruction, register_to_test: RegisterToTest) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x84),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x84),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x84),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, true);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_loading_register_zero_page(opcode: Instruction, register_to_test: RegisterToTest) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x42;
    memory[0x0042] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x37),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x37),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x37),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_loading_register_zero_page_plus_register(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_being_added {
        RegisterToTest::X => cpu.x_register = 5,
        RegisterToTest::Y => cpu.y_register = 5,
        _ => {}
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x42;
    memory[0x0047] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x37),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x37),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x37),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_loading_register_zero_page_plus_register_when_wrap(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_being_added {
        RegisterToTest::X => cpu.x_register = 0xFF,
        RegisterToTest::Y => cpu.y_register = 0xFF,
        _ => {}
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x80;
    memory[0x007F] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x37),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x37),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x37),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_loading_register_absolute(opcode: Instruction, register_to_test: RegisterToTest) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x80;
    memory[0xFFFE] = 0x44;
    memory[0x4480] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x37),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x37),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x37),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_loading_register_absolute_plus_register(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_being_added {
        RegisterToTest::X => cpu.x_register = 1,
        RegisterToTest::Y => cpu.y_register = 1,
        _ => {}
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x80;
    memory[0xFFFE] = 0x44;
    memory[0x4481] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x37),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x37),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x37),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_loading_register_absolute_plus_register_when_crossing_page_boundary(
    opcode: Instruction,
    register_to_test: RegisterToTest,
    register_being_added: RegisterToTest,
) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    match register_being_added {
        RegisterToTest::X => cpu.x_register = 0xFF,
        RegisterToTest::Y => cpu.y_register = 0xFF,
        _ => {}
    }

    memory[0xFFFC] = opcode as Byte;
    memory[0xFFFD] = 0x02;
    memory[0xFFFE] = 0x44;
    memory[0x4501] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    match register_to_test {
        RegisterToTest::A => assert_eq!(cpu.a_register, 0x37),
        RegisterToTest::X => assert_eq!(cpu.x_register, 0x37),
        RegisterToTest::Y => assert_eq!(cpu.y_register, 0x37),
    }
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn lda_immediate_can_load_value() {
    test_loading_register_immediate(Instruction::InsLdaIm, RegisterToTest::A);
}

#[test]
fn lda_immediate_can_affect_zero_flag() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0x44;
    memory[0xFFFC] = Instruction::InsLdaIm as Byte;
    memory[0xFFFD] = 0x0;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));

    assert_eq!(cpu.a_register, 0x0);
    assert_eq!(cpu.status.zero, true);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn lda_zero_page_can_load_value() {
    test_loading_register_zero_page(Instruction::InsLdaZp, RegisterToTest::A);
}

#[test]
fn lda_zero_page_x_can_load_value() {
    test_loading_register_zero_page_plus_register(
        Instruction::InsLdaZpX,
        RegisterToTest::A,
        RegisterToTest::X,
    );
}

#[test]
fn lda_zero_page_x_can_load_value_when_wrap() {
    test_loading_register_zero_page_plus_register_when_wrap(
        Instruction::InsLdaZpX,
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
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.x_register = 0x04;

    memory[0xFFFC] = Instruction::InsLdaIndX as Byte;
    memory[0xFFFD] = 0x02;
    memory[0x0006] = 0x00;
    memory[0x0007] = 0x80;
    memory[0x8000] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    assert_eq!(cpu.a_register, 0x37);
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_value() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.y_register = 0x04;

    memory[0xFFFC] = Instruction::InsLdaIndY as Byte;
    memory[0xFFFD] = 0x02;
    memory[0x0002] = 0x00;
    memory[0x0003] = 0x80;
    memory[0x8004] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));

    assert_eq!(cpu.a_register, 0x37);
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_value_when_crossing_page_boundary() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.y_register = 0xFF;

    memory[0xFFFC] = Instruction::InsLdaIndY as Byte;
    memory[0xFFFD] = 0x02;
    memory[0x0002] = 0x01;
    memory[0x0003] = 0x80;
    memory[0x8100] = 0x37;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    assert_eq!(cpu.a_register, 0x37);
    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, false);

    verify_unmodified_flags(&cpu, &cpu_copy);
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
    test_loading_register_zero_page_plus_register(
        Instruction::InsLdxZpy,
        RegisterToTest::X,
        RegisterToTest::Y,
    );
}

#[test]
fn ldx_zero_page_y_can_load_value_when_wrap() {
    test_loading_register_zero_page_plus_register_when_wrap(
        Instruction::InsLdxZpy,
        RegisterToTest::X,
        RegisterToTest::Y,
    );
}

#[test]
fn ldx_absolute_can_load_value() {
    test_loading_register_absolute(Instruction::InsLdxAbs, RegisterToTest::X);
}

#[test]
fn ldx_absolute_y_can_load_value() {
    test_loading_register_absolute_plus_register(
        Instruction::InsLdxAbsY,
        RegisterToTest::X,
        RegisterToTest::Y,
    );
}

#[test]
fn ldx_absolute_y_can_load_value_when_crossing_page_boundary() {
    test_loading_register_absolute_plus_register_when_crossing_page_boundary(
        Instruction::InsLdxAbsY,
        RegisterToTest::X,
        RegisterToTest::Y,
    );
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
    test_loading_register_zero_page_plus_register(
        Instruction::InsLdyZpX,
        RegisterToTest::Y,
        RegisterToTest::X,
    );
}

#[test]
fn ldy_zero_page_x_can_load_value_when_wrap() {
    test_loading_register_zero_page_plus_register_when_wrap(
        Instruction::InsLdyZpX,
        RegisterToTest::Y,
        RegisterToTest::X,
    );
}

#[test]
fn ldy_absolute_can_load_value() {
    test_loading_register_absolute(Instruction::InsLdyAbs, RegisterToTest::Y);
}

#[test]
fn ldy_absolute_x_can_load_value() {
    test_loading_register_absolute_plus_register(
        Instruction::InsLdyAbsX,
        RegisterToTest::Y,
        RegisterToTest::X,
    );
}

#[test]
fn ldy_absolute_x_can_load_value_when_crossing_page_boundary() {
    test_loading_register_absolute_plus_register_when_crossing_page_boundary(
        Instruction::InsLdyAbsX,
        RegisterToTest::Y,
        RegisterToTest::X,
    );
}
