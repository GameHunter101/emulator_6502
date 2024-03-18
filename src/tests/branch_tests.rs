use crate::{
    cpu::{Byte, Word, CPU, SByte},
    instructions::Instruction,
    memory::Memory,
};

// BEQ
#[test]
fn beq_can_branch_forwards_when_zero_is_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.zero = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBeq as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

#[test]
fn beq_does_not_branch_forwards_when_zero_is_not_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.zero = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBeq as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.program_counter, 0xFF02);
    assert_eq!(cpu.status, cpu_copy.status);
}

#[test]
fn beq_can_branch_forwards_into_new_page_when_zero_is_set() {
    let mut cpu = CPU::reset(Some(0xFEFD));
    let mut memory = Memory::initialize();

    cpu.status.zero = true;

    let cpu_copy = cpu;

    memory[0xFEFD] = Instruction::InsBeq as Byte;
    memory[0xFEFE] = 0x01;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.program_counter, 0xFF00);
    assert_eq!(cpu.status, cpu_copy.status);
}

#[test]
fn beq_can_branch_backwards_when_zero_is_set() {
    let mut cpu = CPU::reset(Some(0xFFCC));
    let mut memory = Memory::initialize();

    cpu.status.zero = true;

    let cpu_copy = cpu;

    memory[0xFFCC] = Instruction::InsBeq as Byte;
    memory[0xFFCD] = -0x02_i8 as Byte;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFFCC);
    assert_eq!(cpu.status, cpu_copy.status);
}
