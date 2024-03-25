use crate::{
    cpu::{Byte, SByte, Word, CPU},
    instructions::Instruction,
    memory::Memory,
};

fn verify_unmodified_flags(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(
        cpu.status.interupt_disable,
        cpu_copy.status.interupt_disable
    );
    assert_eq!(cpu.status.decimal_mode, cpu_copy.status.decimal_mode);
    assert_eq!(cpu.status.break_command, cpu_copy.status.break_command);
}

#[derive(Debug)]
struct AdcTestData {
    carry_before: bool,
    lhs: Byte,
    rhs: Byte,
    expected_answer: Byte,
    expect_c: bool,
    expect_z: bool,
    expect_n: bool,
    expect_v: bool,
}

fn test_im(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcIm as Byte;
    memory[0xFF01] = data.rhs;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_zp(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcZp as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = data.rhs;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_zp_x(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x05;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0047] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcAbs as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_x(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x05;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcAbsX as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8005] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_x_page_cross(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.x_register = 0xFF;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcAbsX as Byte;
    memory[0xFF01] = 0x01;
    memory[0xFF02] = 0x80;
    memory[0x8100] = data.rhs;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_y(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.y_register = 0x05;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcAbsY as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8005] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_y_page_cross(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.y_register = 0xFF;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcAbsY as Byte;
    memory[0xFF01] = 0x01;
    memory[0xFF02] = 0x80;
    memory[0x8100] = data.rhs;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_ind_x(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x05;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcIndX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0047] = 0x92;
    memory[0x0048] = 0xAC;
    memory[0xAC92] = data.rhs;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_ind_y(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.y_register = 0x05;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcIndY as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0x92;
    memory[0x0043] = 0xAC;
    memory[0xAC97] = data.rhs;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_ind_y_page_cross(data: AdcTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = data.carry_before;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;
    cpu.status.overflow = !data.expect_v;

    cpu.a_register = data.lhs;
    cpu.y_register = 0xFF;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsAdcIndY as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0x92;
    memory[0x0043] = 0xAC;
    memory[0xAD91] = data.rhs;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(cpu.a_register, data.expected_answer);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);
    assert_eq!(cpu.status.overflow, data.expect_v);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// Immediate
#[test]
fn adc_immediate_can_add_two_unsigned_numbers() {
    test_im(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_immediate_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_im(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Zero Page
#[test]
fn adc_zero_page_can_add_two_unsigned_numbers() {
    test_zp(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_zero_page_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_zp(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Zero Page X
#[test]
fn adc_zero_page_x_can_add_two_unsigned_numbers() {
    test_zp_x(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_zero_page_x_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_zp_x(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Absolute
#[test]
fn adc_absolute_zero_plus_zero_equals_zero() {
    test_abs(AdcTestData {
        carry_before: false,
        lhs: 0x0,
        rhs: 0x0,
        expected_answer: 0x0,
        expect_c: false,
        expect_z: true,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_zero_plus_zero_equals_one() {
    test_abs(AdcTestData {
        carry_before: true,
        lhs: 0x0,
        rhs: 0x0,
        expected_answer: 0x1,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_ff_plus_one_causes_carry() {
    test_abs(AdcTestData {
        carry_before: false,
        lhs: 0xFF,
        rhs: 0x01,
        expected_answer: 0x0,
        expect_c: true,
        expect_z: true,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_will_set_negative_flag_when_negative_result() {
    test_abs(AdcTestData {
        carry_before: false,
        lhs: 0x0,
        rhs: (-0x01_i8) as Byte,
        expected_answer: (-0x01_i8) as Byte,
        expect_c: false,
        expect_z: false,
        expect_n: true,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_will_set_overflow_flag_when_signed_negative_addition_fails() {
    /*
     * lhs: 10000000 | -128
     * rhs: 11111111 | -1
     * ans: 01111111 | 127
     * C:1, Z:0, N:0, V:1
     * */
    test_abs(AdcTestData {
        carry_before: false,
        lhs: (-128_i8) as Byte,
        rhs: (-0x01_i8) as Byte,
        expected_answer: 127,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: true,
    });
}

#[test]
fn adc_absolute_will_set_overflow_flag_when_signed_negative_addition_succeeds_due_to_carry() {
    /*
     * c:   00000001 | 1
     * lhs: 10000000 | -128
     * rhs: 11111111 | -1
     * ans: 10000000 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_abs(AdcTestData {
        carry_before: true,
        lhs: (-128_i8) as Byte,
        rhs: (-0x01_i8) as Byte,
        expected_answer: (-128_i8) as Byte,
        expect_c: true,
        expect_z: false,
        expect_n: true,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_will_set_overflow_flag_when_signed_positive_addition_fails() {
    /*
     * lhs: 01111111 | 127
     * rhs: 00000001 | 1
     * ans: 10000000 | -128
     * C:0, Z:0, N:0, V:1
     * */
    test_abs(AdcTestData {
        carry_before: false,
        lhs: 127,
        rhs: 1,
        expected_answer: 128,
        expect_c: false,
        expect_z: false,
        expect_n: true,
        expect_v: true,
    });
}

#[test]
fn adc_absolute_can_add_two_unsigned_numbers() {
    test_abs(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_abs(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Absolute X
#[test]
fn adc_absolute_x_can_add_two_unsigned_numbers() {
    test_abs_x(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_x_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_abs_x(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_x_can_add_two_unsigned_numbers_when_crossing_page() {
    test_abs_x_page_cross(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_x_can_add_positive_and_negative_numbers_when_crossing_page() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_abs_x_page_cross(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Absolute Y
#[test]
fn adc_absolute_y_can_add_two_unsigned_numbers() {
    test_abs_y(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_y_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_abs_y(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_y_can_add_two_unsigned_numbers_when_crossing_page() {
    test_abs_y_page_cross(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_absolute_y_can_add_positive_and_negative_numbers_when_crossing_page() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_abs_y_page_cross(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Indirect X
#[test]
fn adc_indirect_x_can_add_two_unsigned_numbers() {
    test_ind_x(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_indirect_x_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_ind_x(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

// Indirect Y
#[test]
fn adc_indirect_y_can_add_two_unsigned_numbers() {
    test_ind_y(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_indirect_y_can_add_positive_and_negative_numbers() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_ind_y(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_indirect_y_can_add_two_unsigned_numbers_when_crossing_page() {
    test_ind_y_page_cross(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: 17,
        expected_answer: 38,
        expect_c: false,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}

#[test]
fn adc_indirect_y_can_add_positive_and_negative_numbers_when_crossing_page() {
    /*
     * lhs: 00010100 | 20 
     * rhs: 11101111 | -17
     * ans: 00010011 | -128
     * C:1, Z:0, N:0, V:0
     * */
    test_ind_y_page_cross(AdcTestData {
        carry_before: true,
        lhs: 20,
        rhs: (-17_i8) as Byte,
        expected_answer: 4,
        expect_c: true,
        expect_z: false,
        expect_n: false,
        expect_v: false,
    });
}
