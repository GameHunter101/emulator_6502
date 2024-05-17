use crate::{
    cpu::{Byte, Word, CPU},
    memory::Memory,
};

/*
* * = $1000
* lda #$xFF
*
* start
* sta $90
* sta $8000
* eor #$CC
* jmp start
* */

static TEST_PROGRAM: [Byte; 14] = [
    0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
];

/*
* *=$1000
*
* lda #00
* sta $42
*
* start
* inc $42
* lda $42
* inx
* jmp start
* */
static INC_MEMORY_PROGRAM: [Byte; 14] = [
    0x00, 0x10, 0xA9, 0x00, 0x85, 0x42, 0xE6, 0x42, 0xA5, 0x42, 0xE8, 0x4C, 0x04, 0x10,
];
/*
* *=$1000
*
* loop
* lda #0
* beq loop
* */
static BEQ_LOOP_PROGRAM: [Byte; 6] = [0x00, 0x10, 0xA9, 0x00, 0xF0, 0xFC];

/*
* * =$1000
*
* lda #0
* clc
* loop
*     adc #8
*     cmp #24
*     bne loop
* ldx #20
* */
static COMPARISON_LOOP_PROGRAM: [Byte; 13] = [
    0x00, 0x10, 0xA9, 0x00, 0x18, 0x69, 0x08, 0xC9, 0x18, 0xD0, 0xFA, 0xA2, 0x14,
];

#[test]
fn test_loading_program_into_memory() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    cpu.load_program(&TEST_PROGRAM, 14, &mut memory);

    assert_eq!(
        &memory[0x1000_usize..(0x1000 + 12) as usize],
        &TEST_PROGRAM[2..]
    );
}

#[test]
fn test_executing_program() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    let start_address = cpu.load_program(&TEST_PROGRAM, 14, &mut memory);
    cpu.program_counter = start_address;

    let mut clock = 1000;
    while clock > 0 {
        clock -= cpu.execute(1, &mut memory).unwrap();
        println!("{cpu}");
    }
}

#[test]
fn test_executing_inc_memory_program() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    let start_address = cpu.load_program(&INC_MEMORY_PROGRAM, 14, &mut memory);
    cpu.program_counter = start_address;

    let mut clock = 1000;
    while clock > 0 {
        clock -= cpu.execute(1, &mut memory).unwrap();
        println!("{cpu}");
    }
}

#[test]
fn test_executing_beq_loop_program() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    let start_address = cpu.load_program(&BEQ_LOOP_PROGRAM, 6, &mut memory);
    cpu.program_counter = start_address;

    let mut clock = 1000;
    while clock > 0 {
        clock -= cpu.execute(1, &mut memory).unwrap();
        println!("{cpu}");
    }
}

/* #[test]
fn test_executing_comparison_loop_program() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    let start_address = cpu.load_program(&COMPARISON_LOOP_PROGRAM, 13, &mut memory);
    cpu.program_counter = start_address;

    let mut clock = 1000;
    while clock > 0 {
        let did_execute = cpu.execute(1, &mut memory);
        if let Ok(executed_cycles ) = did_execute {
            clock -= executed_cycles;
            println!("EXECUTED {executed_cycles} CYCLES");
            println!("{cpu}");
            continue;
        }
        break;
    }
} */

// #[test]
fn test_6502_test_program() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    let start_address = cpu.load_program(&BEQ_LOOP_PROGRAM, 6, &mut memory);
    cpu.program_counter = start_address;

    let slice = &mut memory[0x000A..(0x000A + 65526)];
    let path = std::path::Path::new(&std::env::current_dir().unwrap()).join("assembly/6502_functional_test.bin");
    let file = std::fs::read(path).unwrap();
    slice.copy_from_slice(&file);

    loop {
        cpu.execute(1, &mut memory).unwrap();
        // if cpu.program_counter == 0x63a {
            println!("Val at stack: {}", &memory[cpu.stack_pointer_to_address()]);
        // }
        // println!("{cpu}");
    }
}
