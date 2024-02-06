use cpu::{CPU, Instructions, Byte};
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let mut memory = Memory::initialize();
    let mut cpu= CPU::reset();

    memory[0xFFFC] = Instructions::InsLdaZp as Byte;
    memory[0xFFFD] = 0x42;
    memory[0x0042] = 0x84;

    cpu.execute(3, &mut memory).unwrap();

}
