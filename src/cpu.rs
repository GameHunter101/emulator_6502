#![allow(unused)]
use crate::memory::Memory;

pub type Byte = u8;
pub type Word = u16;

#[derive(Debug, Clone, Copy)]
pub struct CPU {
    // Addresses
    pub program_counter: Word,
    pub stack_pointer: Byte,

    // Registers
    pub a_register: Byte,
    pub x_register: Byte,
    pub y_register: Byte,

    // Status flags
    pub carry: bool,
    pub zero: bool,
    pub interupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CPU {
    pub const fn reset() -> CPU {
        CPU {
            program_counter: 0xFFFC,
            stack_pointer: 0,
            a_register: 0,
            x_register: 0,
            y_register: 0,
            carry: false,
            zero: false,
            interupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn lda_set_status(&mut self) {
        self.zero = self.a_register == 0;
        self.negative = (self.a_register & 0b10000000) > 0;
    }

    pub fn execute(&mut self, cycles: i32, memory: &mut Memory) -> Result<i32, InstructionsError> {
        let cycles_requested = cycles as i32;
        let mut cycles = cycles;
        while cycles > 0 {
            let instruction_byte = Instructions::try_from(self.fetch_byte(&mut cycles, memory))?;
            match instruction_byte {
                Instructions::InsLdaIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instructions::InsLdaZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instructions::InsLdaZpx => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instructions::InsJsr => {
                    let subroutine_address = self.fetch_word(&mut cycles, memory);
                    memory.write_word(self.program_counter-1, self.stack_pointer as Word + 0xFF, &mut cycles);
                    self.stack_pointer += 1;
                    self.program_counter = subroutine_address;
                    cycles -= 1;
                }
                Instructions::InsLdaAbs => {}
            }
        }
        Ok(cycles_requested - cycles)
    }

    pub fn fetch_byte(&mut self, cycles: &mut i32, memory: &mut Memory) -> Byte {
        let data: Byte = memory[self.program_counter];
        self.program_counter += 1;
        *cycles -= 1;
        data
    }

    pub fn fetch_word(&mut self, cycles: &mut i32, memory: &mut Memory) -> Word {
        // 6502 is little endian
        let low_byte = memory[self.program_counter] as Word;
        self.program_counter += 1;
        *cycles -= 1;

        let high_byte= (memory[self.program_counter] as Word) << 8;
        self.program_counter += 1;
        *cycles -= 1;

        let data: Word = low_byte | high_byte;
        data
    }

    pub fn read_byte(&self, cycles: &mut i32, memory: &mut Memory, address: Byte) -> Byte {
        let data: Byte = memory[address];
        *cycles -= 1;
        data
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionsError {
    InstructionDoesntExist(Byte),
}

pub enum Instructions {
    InsLdaIm = 0xA9,
    InsLdaZp = 0xA5,
    InsLdaZpx = 0xB5,
    InsLdaAbs = 0xAD,
    InsJsr = 0x20,
}

impl TryFrom<Byte> for Instructions {
    type Error = InstructionsError;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            0xA9 => Ok(Self::InsLdaIm),
            0xA5 => Ok(Self::InsLdaZp),
            0xB5 => Ok(Self::InsLdaZpx),
            0xAD => Ok(Self::InsLdaAbs),
            0x20 => Ok(Self::InsJsr),
            _ => Err(InstructionsError::InstructionDoesntExist(value)),
        }
    }
}
