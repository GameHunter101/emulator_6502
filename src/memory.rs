use std::ops::{Index, IndexMut, Range};

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

impl Index<Range<usize>> for Memory {
    type Output = [Byte];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}
