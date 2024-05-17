use crate::{
    cpu::{Byte, ProcessorFlags, CPU},
    instructions::{Instruction, InstructionsError},
    memory::Memory,
};

use super::load_tests::RegisterToTest;

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(cpu.status.carry, cpu_copy.status.carry);
    assert_eq!(
        cpu.status.interupt_disable,
        cpu_copy.status.interupt_disable
    );
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
}

#[test]
fn tsx_can_transfer_stack_pointer() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status = 0xFF.into();
    cpu.x_register = 0x00;
    cpu.stack_pointer = 0x01;

    memory[0xFF00] = Instruction::InsTsx as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x01);
    assert!(!cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn tsx_can_transfer_zero_stack_pointer() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status = 0xFF.into();
    cpu.status.zero = false;
    cpu.x_register = 0x00;
    cpu.stack_pointer = 0x00;

    memory[0xFF00] = Instruction::InsTsx as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0x00);
    assert!(cpu.status.zero);
    assert!(!cpu.status.negative);
}

#[test]
fn tsx_can_transfer_negative_stack_pointer() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status = 0xFF.into();
    cpu.status.negative = false;
    cpu.x_register = 0x00;
    cpu.stack_pointer = 0b10000000;

    memory[0xFF00] = Instruction::InsTsx as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.x_register, 0b10000000);
    assert!(!cpu.status.zero);
    assert!(cpu.status.negative);
}

#[test]
fn txs_can_transfer_x_register_to_stack_pointer() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.x_register = 0xFF;
    cpu.stack_pointer = 0x00;

    memory[0xFF00] = Instruction::InsTxs as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.status, cpu_copy.status);
}

#[test]
fn pha_can_push_a_register_to_stack() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.a_register = 0x42;

    memory[0xFF00] = Instruction::InsPha as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(memory[cpu.stack_pointer_to_address() + 1], cpu.a_register);
    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.stack_pointer, 0xFE);
}

#[test]
fn pla_can_pull_value_from_stack_to_a_register() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.a_register = 0x00;
    cpu.stack_pointer = 0xFE;

    memory[0x01FF] = 0x42;
    memory[0xFF00] = Instruction::InsPla as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, 0x42);
    assert_eq!(cpu.stack_pointer, 0xFF);
}

#[test]
fn pla_can_pull_zero_value_from_stack_to_a_register() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.zero = false;
    cpu.status.negative = true;
    cpu.a_register = 0x42;
    cpu.stack_pointer = 0xFE;

    memory[0x01FF] = 0x00;
    memory[0xFF00] = Instruction::InsPla as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, 0x00);
    assert!(!cpu.status.negative);
    assert!(cpu.status.zero);
    assert_eq!(cpu.stack_pointer, 0xFF);
}

#[test]
fn pla_can_pull_negative_value_from_stack_to_a_register() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.zero = true;
    cpu.status.negative = false;
    cpu.a_register = 0x42;
    cpu.stack_pointer = 0xFE;

    memory[0x01FF] = 0xF0;
    memory[0xFF00] = Instruction::InsPla as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, 0xF0);
    assert!(cpu.status.negative);
    assert!(!cpu.status.zero);
    assert_eq!(cpu.stack_pointer, 0xFF);
}

#[test]
fn php_can_push_zero_to_stack() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status = 0x0.into();

    memory[0xFF00] = Instruction::InsPhp as Byte;

    let mut cpu_copy = cpu;

    cpu_copy.status.unused = true;
    cpu_copy.status.break_command = true;
    cpu_copy.status.interupt_disable = true;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(memory[cpu.stack_pointer_to_address() + 1], 0b00110000);
    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.stack_pointer, 0xFE);
}

#[test]
fn php_can_push_processor_status_to_stack() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status = 0xFF.into();


    memory[0xFF00] = Instruction::InsPhp as Byte;

    let mut cpu_copy = cpu;

    cpu_copy.status.unused = true;
    cpu_copy.status.break_command = true;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(memory[cpu.stack_pointer_to_address() + 1], 0xFF);
    assert_eq!(cpu.status, cpu_copy.status);
    assert_eq!(cpu.stack_pointer, 0xFE);
}

#[test]
fn plp_can_pull_processor_status_from_stack() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status = 0x00.into();
    cpu.stack_pointer = 0xFE;

    memory[0x01FF] = 0x42;
    memory[0xFF00] = Instruction::InsPlp as Byte;

    let cpu_copy = cpu;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(u8::from(cpu.status), 0x42);
}
