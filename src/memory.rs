use crate::cpu::{Byte, Word};
const MAX_MEM: usize = 1024 * 64;

#[derive(Debug)]
pub struct Memory {
    data: [Byte; MAX_MEM],
}

impl Memory {
    pub fn initialize() -> Memory {
        Memory { data: [0; MAX_MEM] }
    }
}
