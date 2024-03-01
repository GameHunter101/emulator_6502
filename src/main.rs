#![allow(unused)]
use cpu::{CPU, Instruction, Byte};
use memory::Memory;

mod cpu;
mod memory;

#[cfg(test)]
mod tests {
    pub mod load_tests;
    pub mod store_tests;
    pub mod jumps_and_calls_tests;
    pub mod stack_operations_tests;
    pub mod logical_ops_tests;
}

fn main() {
}
