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
    pub mod loading_program;
    pub mod logical_ops_tests;
    pub mod stack_operations_tests;
    pub mod store_tests;
    pub mod transfer_register_tests;
    pub mod inc_dec_tests;
    pub mod branch_tests;
    pub mod status_changes_tests;
    pub mod miscellaneous_tests;
    pub mod add_subtract_with_carry_tests;
    pub mod compare_register_tests;
    pub mod shifts_tests;
}

fn main() {}
