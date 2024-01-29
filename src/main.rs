use cpu::CPU;
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let memory = Memory::initialize();
    let cpu= CPU::reset();

}
