use crate::{
    cpu::{Byte, SByte, Word, CPU},
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

// BRK
#[test]
fn brk_will_load_program_counter_from_interrupt_vector() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBrk as Byte;
    memory[0xFFFE] = 0x00;
    memory[0xFFFF] = 0x80;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(cpu.program_counter, 0x8000);
}

#[test]
fn brk_will_load_program_counter_from_interrupt_vector_again() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBrk as Byte;
    memory[0xFFFE] = 0x00;
    memory[0xFFFF] = 0x90;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(cpu.program_counter, 0x9000);
}

#[test]
fn brk_will_set_break_flag() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    cpu.status.break_command = false;

    memory[0xFF00] = Instruction::InsBrk as Byte;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert!(cpu.status.break_command);
}

#[test]
fn brk_will_push_3_bytes_onto_stack() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBrk as Byte;
    memory[0xFFFE] = 0x00;
    memory[0xFFFF] = 0x90;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer - 3);
}

#[test]
fn brk_will_push_pc_and_ps_onto_stack() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let mut cpu_copy = cpu;
    cpu_copy.status.unused = true;
    cpu_copy.status.break_command = true;
    let old_sp = cpu_copy.stack_pointer as Word;

    memory[0xFF00] = Instruction::InsBrk as Byte;

    let cycles = cpu.execute(7, &mut memory);

    assert_eq!(cycles, Ok(7));
    assert_eq!(memory[(0x100 | old_sp)], 0xFF);
    assert_eq!(memory[(0x100 | old_sp) - 1], 0x01);
    assert_eq!(memory[(0x100 | old_sp) - 2], cpu_copy.status.into_u8());
    assert!(cpu.status.interupt_disable);
}

// RTI

#[test]
fn rti_can_can_return_from_interrupt_leaving_cup_in_original_state() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBrk as Byte;
    memory[0xFFFE] = 0x00;
    memory[0xFFFF] = 0x80;
    memory[0x8000] = Instruction::InsRti as Byte;

    // let cycles = cpu.execute(13, &mut memory);
    let brk_cycles = cpu.execute(7, &mut memory);
    let rti_cycles = cpu.execute(6, &mut memory);

    // assert_eq!(cycles, Ok(13));
    assert_eq!(brk_cycles, Ok(7));
    assert_eq!(rti_cycles, Ok(6));
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);
    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.program_counter, 0xFF01);
}
