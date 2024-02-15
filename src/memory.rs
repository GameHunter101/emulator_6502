use std::ops::{Index, IndexMut};

use crate::cpu::{Byte, Word};
const MAX_MEM: usize = 1024 * 64;

#[derive(Debug)]
pub struct Memory {
    data: [Byte; MAX_MEM],
}

impl Memory {
    pub const fn initialize() -> Memory {
        Memory { data: [0; MAX_MEM] }
    }

    pub fn write_word(&mut self, data: Word, address: Word, cycles: &mut i32) {
        let data_bytes = data.to_le_bytes();
        self[address] = data_bytes[0];
        *cycles -= 1;
        self[address + 1] = data_bytes[1];
        *cycles -=1;
    }

    pub fn write_byte(&mut self, data: Byte, address: Word, cycles: &mut i32) {
        self[address] = data;
        *cycles -= 1;
    }
}

impl Index<Word> for Memory {
    type Output = Byte;
    fn index(&self, index: Word) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<Word> for Memory {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl Index<Byte> for Memory {
    type Output = Byte;
    fn index(&self, index: Byte) -> &Self::Output {
        &self.data[index as usize]
    }
}
