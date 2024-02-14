#![allow(unused)]
use cpu::{CPU, Instruction, Byte};
use memory::Memory;

mod cpu;
mod memory;

#[cfg(test)]
mod tests {
    pub mod load_tests;
    pub mod store_tests;
}

fn main() {
}
