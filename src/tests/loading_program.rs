use crate::{
    cpu::{Byte, Word, CPU},
    memory::Memory,
};

static TEST_PROGRAM: [Byte; 14] = [
    0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
];

#[test]
fn test_loading_program_into_memory() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    cpu.load_program(&TEST_PROGRAM, 14, &mut memory);
    
    assert_eq!(&memory[0x1000_usize..(0x1000+12) as usize], &TEST_PROGRAM[2..]);

}

#[test]
fn test_executing_program() {
    let mut cpu = CPU::reset(Some(0x0FFF));
    let mut memory = Memory::initialize();

    let start_address = cpu.load_program(&TEST_PROGRAM, 14, &mut memory);
    cpu.program_counter = start_address;

    for _ in 0..10000 {
        cpu.execute(20, &mut memory);
        println!("A: {:0x}, PC: {:0x}, SP: {:0x}", cpu.a_register, cpu.program_counter, cpu.stack_pointer);
    }
}
