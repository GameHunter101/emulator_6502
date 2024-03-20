use crate::{
    cpu::{Byte, Word, CPU, SByte},
    instructions::Instruction,
    memory::Memory,
};

// NOP
#[test]
fn nop_will_do_nothing_but_consume_cycle() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsNop as Byte;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.program_counter, 0xFF01);
    assert_eq!(cpu.a_register, cpu_copy.a_register);
    assert_eq!(cpu.x_register, cpu_copy.x_register);
    assert_eq!(cpu.y_register, cpu_copy.y_register);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);
}
