use crate::{
    cpu::{Byte, Instruction, InstructionsError, CPU},
    memory::Memory,
};

enum LogicalOperator {
    AND,
    EOR,
    ORA,
}

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

fn do_logical_op(lhs: Byte, rhs: Byte, logical_operator: LogicalOperator) -> Byte {
    match logical_operator {
        LogicalOperator::AND => lhs & rhs,
        LogicalOperator::EOR => lhs ^ rhs,
        LogicalOperator::ORA => lhs | rhs,
    }
}

fn test_logical_op_on_a_register_immediate(logical_operator: LogicalOperator) {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.a_register = 0xCC;

    match logical_operator {
        LogicalOperator::AND => memory[0xFFFC] = Instruction::InsAndIm as Byte,
        LogicalOperator::EOR => memory[0xFFFC] = Instruction::InsEorIm as Byte,
        LogicalOperator::ORA => memory[0xFFFC] = Instruction::InsOraIm as Byte,
    }

    memory[0xFFFD] = 0x84;

    let cpu_copy = cpu.clone();

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));

    assert_eq!(cpu.a_register, do_logical_op(0xCC, 0x84, logical_operator));

    assert_eq!(cpu.status.zero, false);
    assert_eq!(cpu.status.negative, true);

    verify_unmodified_flags(&cpu, &cpu_copy);
}
