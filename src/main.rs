#![allow(unused)]
use cpu::{Byte, CPU};
use instructions::{Instruction, InstructionsError};
use memory::Memory;

pub mod cpu;
pub mod instructions;
pub mod memory;

#[cfg(test)]
mod tests {
    pub mod jumps_and_calls_tests;
    pub mod load_tests;
    pub mod logical_ops_tests;
    pub mod stack_operations_tests;
    pub mod store_tests;
}

fn main() {
    let mut cpu = CPU::reset(Some(0xFF00));
    let mut memory = Memory::initialize();

    let program = vec![
        0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
    ];

    cpu.load_program(program, 14);

    for _ in 0..10000 {
        cpu.execute(20, &mut memory);
    }
}
