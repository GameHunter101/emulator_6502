use crate::{
    cpu::{Byte, Instructions, InstructionsError, CPU},
    memory::Memory,
};

static mut CPU: CPU = CPU::reset();
static mut MEMORY: Memory = Memory::initialize();

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
        let cycles = CPU.execute(0, &mut MEMORY).unwrap();

        assert_eq!(cycles, 0);
    }
}

#[test]
fn executing_bad_instruction_does_not_loop() {
    unsafe {
        MEMORY[0xFFFC] = 0x0;
        MEMORY[0xFFFD] = 0x0;

        let cycles = CPU.execute(4, &mut MEMORY);

        assert_eq!(
            cycles.is_err_and(|err| err == InstructionsError::InstructionDoesntExist(0x0)),
            true
        );
        CPU = CPU::reset();
    }
}

#[test]
fn lda_immediate_can_load_value() {
    unsafe {
        MEMORY[0xFFFC] = Instructions::InsLdaIm as Byte;
        MEMORY[0xFFFD] = 0x84;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(2, &mut MEMORY).unwrap();

        assert_eq!(cycles, 2);

        assert_eq!(CPU.a_register, 0x84);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, true);

        verify_unmodified_flags_lda(&cpu_copy);

        CPU = CPU::reset();
    }
}

#[test]
fn lda_immediate_can_affect_zero_flag() {
    unsafe {
        CPU.a_register = 0x44;
        MEMORY[0xFFFC] = Instructions::InsLdaIm as Byte;
        MEMORY[0xFFFD] = 0x0;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(2, &mut MEMORY).unwrap();

        assert_eq!(cycles, 2);

        assert_eq!(CPU.a_register, 0x0);
        assert_eq!(CPU.zero, true);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);

        CPU = CPU::reset();
    }
}

#[test]
fn lda_zero_page_can_load_value() {
    unsafe {
        MEMORY[0xFFFC] = Instructions::InsLdaZp as Byte;
        MEMORY[0xFFFD] = 0x42;
        MEMORY[0x0042] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(3, &mut MEMORY).unwrap();

        assert_eq!(cycles, 3);

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);

        CPU = CPU::reset();
    }
}

#[test]
fn lda_zero_page_x_can_load_value() {
    unsafe {
        CPU.x_register = 5;

        MEMORY[0xFFFC] = Instructions::InsLdaZpx as Byte;
        MEMORY[0xFFFD] = 0x42;
        MEMORY[0x0047] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY).unwrap();

        assert_eq!(cycles, 4);

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);

        CPU = CPU::reset();
    }
}

#[test]
fn lda_zero_page_x_can_load_value_when_wrap() {
    unsafe {
        CPU.x_register = 0xFF;

        MEMORY[0xFFFC] = Instructions::InsLdaZpx as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0x007F] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY).unwrap();

        assert_eq!(cycles, 4);

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);

        CPU = CPU::reset();
    }
}

#[test]
fn lda_absolute_can_load_value() {
    unsafe {
        MEMORY[0xFFFC] = Instructions::InsLdaAbs as Byte;
        MEMORY[0xFFFD] = 0x80;
        MEMORY[0x007F] = 0x44;
        MEMORY[0x4480] = 0x37;

        let cpu_copy = CPU.clone();

        let cycles = CPU.execute(4, &mut MEMORY).unwrap();

        assert_eq!(cycles, 4);

        assert_eq!(CPU.a_register, 0x37);
        assert_eq!(CPU.zero, false);
        assert_eq!(CPU.negative, false);

        verify_unmodified_flags_lda(&cpu_copy);

        CPU = CPU::reset();
    }
}
