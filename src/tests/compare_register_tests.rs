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
    assert_eq!(cpu.status.overflow, cpu_copy.status.overflow);
}

#[derive(Debug)]
enum CompareRegister {
    A,
    X,
    Y,
}

#[derive(Debug)]
struct CmpTestData {
    lhs: Byte,
    rhs: Byte,
    expect_c: bool,
    expect_z: bool,
    expect_n: bool,
}

fn test_im(data: CmpTestData, register_to_test: CompareRegister) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    match register_to_test {
        CompareRegister::A => cpu.a_register = data.lhs,
        CompareRegister::X => cpu.x_register = data.lhs,
        CompareRegister::Y => cpu.y_register = data.lhs,
    }

    let cpu_copy = cpu;

    memory[0xFF00] = match register_to_test {
        CompareRegister::A => Instruction::InsCmpIm as Byte,
        CompareRegister::X => Instruction::InsCpxIm as Byte,
        CompareRegister::Y => Instruction::InsCpyIm as Byte,
    };
    memory[0xFF01] = data.rhs;

    let cycles = cpu.execute(2, &mut memory);

    assert_eq!(cycles, Ok(2));
    assert_eq!(
        match register_to_test {
            CompareRegister::A => cpu.a_register,
            CompareRegister::X => cpu.x_register,
            CompareRegister::Y => cpu.y_register,
        },
        data.lhs
    );
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_zp(data: CmpTestData, register_to_test: CompareRegister) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    match register_to_test {
        CompareRegister::A => cpu.a_register = data.lhs,
        CompareRegister::X => cpu.x_register = data.lhs,
        CompareRegister::Y => cpu.y_register = data.lhs,
    }

    let cpu_copy = cpu;

    memory[0xFF00] = match register_to_test {
        CompareRegister::A => Instruction::InsCmpZp as Byte,
        CompareRegister::X => Instruction::InsCpxZp as Byte,
        CompareRegister::Y => Instruction::InsCpyZp as Byte,
    };
    memory[0xFF01] = 0x42;
    memory[0x0042] = data.rhs;

    let cycles = cpu.execute(3, &mut memory);

    assert_eq!(cycles, Ok(3));
    assert_eq!(
        match register_to_test {
            CompareRegister::A => cpu.a_register,
            CompareRegister::X => cpu.x_register,
            CompareRegister::Y => cpu.y_register,
        },
        data.lhs
    );
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_zp_x(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpZpX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs(data: CmpTestData, register_to_test: CompareRegister) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    match register_to_test {
        CompareRegister::A => cpu.a_register = data.lhs,
        CompareRegister::X => cpu.x_register = data.lhs,
        CompareRegister::Y => cpu.y_register = data.lhs,
    }

    let cpu_copy = cpu;

    memory[0xFF00] = match register_to_test {
        CompareRegister::A => Instruction::InsCmpAbs as Byte,
        CompareRegister::X => Instruction::InsCpxAbs as Byte,
        CompareRegister::Y => Instruction::InsCpyAbs as Byte,
    };
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8000] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(
        match register_to_test {
            CompareRegister::A => cpu.a_register,
            CompareRegister::X => cpu.x_register,
            CompareRegister::Y => cpu.y_register,
        },
        data.lhs
    );
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_x(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpAbsX as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8010] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_x_page_cross(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpAbsX as Byte;
    memory[0xFF01] = 0xF0;
    memory[0xFF02] = 0x80;
    memory[0x8100] = data.rhs;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_y(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.y_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpAbsY as Byte;
    memory[0xFF01] = 0x00;
    memory[0xFF02] = 0x80;
    memory[0x8010] = data.rhs;

    let cycles = cpu.execute(4, &mut memory);

    assert_eq!(cycles, Ok(4));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_abs_y_page_cross(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.y_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpAbsY as Byte;
    memory[0xFF01] = 0xF0;
    memory[0xFF02] = 0x80;
    memory[0x8100] = data.rhs;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_ind_x(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.x_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpIndX as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0052] = 0x80;
    memory[0x0053] = 0x68;
    memory[0x6880] = data.rhs;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_ind_y(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.y_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpIndY as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0x80;
    memory[0x0043] = 0x78;
    memory[0x7890] = data.rhs;

    let cycles = cpu.execute(5, &mut memory);

    assert_eq!(cycles, Ok(5));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

fn test_ind_y_page_cross(data: CmpTestData) {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    cpu.status.carry = !data.expect_c;
    cpu.status.zero = !data.expect_z;
    cpu.status.negative = !data.expect_n;

    cpu.a_register = data.lhs;
    cpu.y_register = 0x10;

    let cpu_copy = cpu;

    memory[0xFF00] = Instruction::InsCmpIndY as Byte;
    memory[0xFF01] = 0x42;
    memory[0x0042] = 0xF0;
    memory[0x0043] = 0x78;
    memory[0x7900] = data.rhs;

    let cycles = cpu.execute(6, &mut memory);

    assert_eq!(cycles, Ok(6));
    assert_eq!(cpu.a_register, data.lhs);
    assert_eq!(cpu.status.carry, data.expect_c);
    assert_eq!(cpu.status.zero, data.expect_z);
    assert_eq!(cpu.status.negative, data.expect_n);

    verify_unmodified_flags(&cpu, &cpu_copy);
}

// CMP

// Immediate
#[test]
fn cmp_immediate_can_compare_two_identical_values() {
    test_im(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_immediate_can_compare_large_positive_to_small_positive() {
    test_im(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_immediate_can_compare_negative_to_positive() {
    test_im(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_immediate_can_compare_two_values_resulting_in_negative() {
    test_im(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::A,
    );
}

// Zero Page
#[test]
fn cmp_zero_page_can_compare_two_identical_values() {
    test_zp(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_zero_page_can_compare_large_positive_to_small_positive() {
    test_zp(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_zero_page_can_compare_negative_to_positive() {
    test_zp(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_zero_page_can_compare_two_values_resulting_in_negative() {
    test_zp(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::A,
    );
}

// Zero Page X

#[test]
fn cmp_zero_page_x_can_compare_two_identical_values() {
    test_zp_x(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_zero_page_x_can_compare_large_positive_to_small_positive() {
    test_zp_x(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_zero_page_x_can_compare_negative_to_positive() {
    test_zp_x(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_zero_page_x_can_compare_two_values_resulting_in_negative() {
    test_zp_x(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

// Absolute
#[test]
fn cmp_absolute_can_compare_two_identical_values() {
    test_abs(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_absolute_can_compare_large_positive_to_small_positive() {
    test_abs(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_absolute_can_compare_negative_to_positive() {
    test_abs(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::A,
    );
}

#[test]
fn cmp_absolute_can_compare_two_values_resulting_in_negative() {
    test_abs(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::A,
    );
}

// Absolute X
#[test]
fn cmp_absolute_x_can_compare_two_identical_values() {
    test_abs_x(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_x_can_compare_large_positive_to_small_positive() {
    test_abs_x(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_x_can_compare_negative_to_positive() {
    test_abs_x(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_x_can_compare_two_values_resulting_in_negative() {
    test_abs_x(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

#[test]
fn cmp_absolute_x_can_compare_two_identical_values_when_crossing_page() {
    test_abs_x_page_cross(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_x_can_compare_large_positive_to_small_positive_when_crossing_page() {
    test_abs_x_page_cross(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_x_can_compare_negative_to_positive_when_crossing_page() {
    test_abs_x_page_cross(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_x_can_compare_two_values_resulting_in_negative_when_crossing_page() {
    test_abs_x_page_cross(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

// Absolute Y
#[test]
fn cmp_absolute_y_can_compare_two_identical_values() {
    test_abs_y(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_y_can_compare_large_positive_to_small_positive() {
    test_abs_y(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_y_can_compare_negative_to_positive() {
    test_abs_y(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_y_can_compare_two_values_resulting_in_negative() {
    test_abs_y(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

#[test]
fn cmp_absolute_y_can_compare_two_identical_values_when_crossing_page() {
    test_abs_y_page_cross(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_y_can_compare_large_positive_to_small_positive_when_crossing_page() {
    test_abs_y_page_cross(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_y_can_compare_negative_to_positive_when_crossing_page() {
    test_abs_y_page_cross(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_absolute_y_can_compare_two_values_resulting_in_negative_when_crossing_page() {
    test_abs_y_page_cross(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

// Indirect X
#[test]
fn cmp_indirect_x_can_compare_two_identical_values() {
    test_ind_x(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_x_can_compare_large_positive_to_small_positive() {
    test_ind_x(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_x_can_compare_negative_to_positive() {
    test_ind_x(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_x_can_compare_two_values_resulting_in_negative() {
    test_ind_x(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

// Absolute Y
#[test]
fn cmp_indirect_y_can_compare_two_identical_values() {
    test_ind_y(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_y_can_compare_large_positive_to_small_positive() {
    test_ind_y(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_y_can_compare_negative_to_positive() {
    test_ind_y(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_y_can_compare_two_values_resulting_in_negative() {
    test_ind_y(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

#[test]
fn cmp_indirect_y_can_compare_two_identical_values_when_crossing_page() {
    test_ind_y_page_cross(CmpTestData {
        lhs: 26,
        rhs: 26,
        expect_c: true,
        expect_z: true,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_y_can_compare_large_positive_to_small_positive_when_crossing_page() {
    test_ind_y_page_cross(CmpTestData {
        lhs: 48,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_y_can_compare_negative_to_positive_when_crossing_page() {
    test_ind_y_page_cross(CmpTestData {
        lhs: 130,
        rhs: 26,
        expect_c: true,
        expect_z: false,
        expect_n: false,
    });
}

#[test]
fn cmp_indirect_y_can_compare_two_values_resulting_in_negative_when_crossing_page() {
    test_ind_y_page_cross(CmpTestData {
        lhs: 8,
        rhs: 26,
        expect_c: false,
        expect_z: false,
        expect_n: true,
    });
}

// CPX

// Immediate
#[test]
fn cpx_immediate_can_compare_two_identical_values() {
    test_im(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_immediate_can_compare_large_positive_to_small_positive() {
    test_im(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_immediate_can_compare_negative_to_positive() {
    test_im(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_immediate_can_compare_two_values_resulting_in_negative() {
    test_im(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::X,
    );
}

// Zero Page
#[test]
fn cpx_zero_page_can_compare_two_identical_values() {
    test_zp(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_zero_page_can_compare_large_positive_to_small_positive() {
    test_zp(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_zero_page_can_compare_negative_to_positive() {
    test_zp(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_zero_page_can_compare_two_values_resulting_in_negative() {
    test_zp(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::X,
    );
}

// Absolute
#[test]
fn cpx_absolute_can_compare_two_identical_values() {
    test_abs(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_absolute_can_compare_large_positive_to_small_positive() {
    test_abs(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_absolute_can_compare_negative_to_positive() {
    test_abs(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::X,
    );
}

#[test]
fn cpx_absolute_can_compare_two_values_resulting_in_negative() {
    test_abs(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::X,
    );
}

// CPY

// Immediate
#[test]
fn cpy_immediate_can_compare_two_identical_values() {
    test_im(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_immediate_can_compare_large_positive_to_small_positive() {
    test_im(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_immediate_can_compare_negative_to_positive() {
    test_im(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_immediate_can_compare_two_values_resulting_in_negative() {
    test_im(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::Y,
    );
}

// Zero Page
#[test]
fn cpy_zero_page_can_compare_two_identical_values() {
    test_zp(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_zero_page_can_compare_large_positive_to_small_positive() {
    test_zp(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_zero_page_can_compare_negative_to_positive() {
    test_zp(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_zero_page_can_compare_two_values_resulting_in_negative() {
    test_zp(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::Y,
    );
}

// Absolute
#[test]
fn cpy_absolute_can_compare_two_identical_values() {
    test_abs(
        CmpTestData {
            lhs: 26,
            rhs: 26,
            expect_c: true,
            expect_z: true,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_absolute_can_compare_large_positive_to_small_positive() {
    test_abs(
        CmpTestData {
            lhs: 48,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_absolute_can_compare_negative_to_positive() {
    test_abs(
        CmpTestData {
            lhs: 130,
            rhs: 26,
            expect_c: true,
            expect_z: false,
            expect_n: false,
        },
        CompareRegister::Y,
    );
}

#[test]
fn cpy_absolute_can_compare_two_values_resulting_in_negative() {
    test_abs(
        CmpTestData {
            lhs: 8,
            rhs: 26,
            expect_c: false,
            expect_z: false,
            expect_n: true,
        },
        CompareRegister::Y,
    );
}
