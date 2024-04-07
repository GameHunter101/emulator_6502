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

#[test]
fn can_jump_to_subroutine_and_jump_back() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    memory[0xFF00] = Instruction::InsJsr as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = Instruction::InsRts as Byte;
    memory[0xFF03] = Instruction::InsLdaIm as Byte;
    memory[0xFF04] = 0x42;

    let cpu_copy = cpu;

    let cycles = cpu.execute(14, &mut memory);

    assert_eq!(cycles, Ok(14));

    assert_eq!(cpu.a_register, 0x42);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn jsr_does_not_affect_processor_status() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    memory[0xFF00] = Instruction::InsJsr as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;

    let cpu_copy = cpu;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));

    assert_eq!(cpu.program_counter, 0x8000);

    assert_eq!(cpu.status, cpu_copy.status);
    assert_ne!(cpu.stack_pointer, cpu_copy.stack_pointer);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn rts_does_not_affect_processor_status() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    memory[0xFF00] = Instruction::InsJsr as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = Instruction::InsRts as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(12, &mut memory);

    assert_eq!(cycles, Ok(12));

    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn jmp_absolute_can_jump_to_new_location() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    memory[0xFF00] = Instruction::InsJmpAbs as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;

    let cpu_copy = cpu;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));

    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

#[test]
fn jmp_indirect_can_jump_to_new_location() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    memory[0xFF00] = Instruction::InsJmpInd as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = 0x00;
    memory[0x8001] = 0x90;

    let cpu_copy = cpu;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.program_counter, 0x9000);

    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);

    verify_unmodified_flags(&cpu, &cpu_copy);
}
