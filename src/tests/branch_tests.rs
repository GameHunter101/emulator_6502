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

// BNE
#[test]
fn bne_can_branch_forwards_when_zero_is_not_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.zero = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBne as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

// BCS
#[test]
fn bcs_can_branch_forwards_when_carry_is_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBne as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

// BCC
#[test]
fn bcc_can_branch_forwards_when_carry_is_not_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBne as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

// BMI
#[test]
fn bmi_can_branch_forwards_when_negative_is_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.negative = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBmi as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

// BPL
#[test]
fn bpl_can_branch_forwards_when_negative_is_not_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.negative = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBpl as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

// BVS
#[test]
fn bvs_can_branch_forwards_when_overflow_is_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.overflow = true;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBvs as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}

// BVC
#[test]
fn bvc_can_branch_forwards_when_overflow_is_not_set() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.overflow = false;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsBvc as Byte;
    memory[0xFF01] = 0x01;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.program_counter, 0xFF03);
    assert_eq!(cpu.status, cpu_copy.status);
}
