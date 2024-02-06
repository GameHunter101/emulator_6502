use cpu::{CPU, Instructions, Byte};
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let mut memory = Memory::initialize();
    let mut cpu= CPU::reset();

    memory[0xFFFC] = Instructions::InsLdaIm as Byte;
    memory[0xFFFD] = 0x42;

    cpu.execute(2, &mut memory).unwrap();

}
