use cpu::{CPU, Instructions, Byte};
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let mut memory = Memory::initialize();
    let mut cpu= CPU::reset();

    memory[0xFFFC] = Instructions::InsJsr as Byte;
    memory[0xFFFD] = 0x42;
    memory[0xFFFE] = 0x42;
    memory[0x4242] = Instructions::InsLdaIm as Byte;
    memory[0x4243] = 0x84;

    cpu.execute(8, &mut memory).unwrap();
    println!("Finished");
}
