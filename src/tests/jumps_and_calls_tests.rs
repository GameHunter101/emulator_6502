use crate::{
    cpu::{Byte, Instruction, InstructionsError, CPU},
    memory::Memory,
};

use super::load_tests::RegisterToTest;

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(cpu.carry, cpu_copy.carry);
    assert_eq!(cpu.interupt_disable, cpu_copy.interupt_disable);
    assert_eq!(cpu.decimal_mode, cpu_copy.decimal_mode);
    assert_eq!(cpu.break_command, cpu_copy.break_command);
    assert_eq!(cpu.overflow, cpu_copy.overflow);
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

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(14, &mut memory);

    assert_eq!(cycles, Ok(14));

    assert_eq!(cpu.a_register, 0x42);

    verify_unmodified_flags(&cpu, &cpu_copy);
}
