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
    
    pub fn ldx_set_status(&mut self) {
        self.zero = self.x_register == 0;
        self.negative = (self.x_register & 0b10000000) > 0;
    }

    pub fn ldy_set_status(&mut self) {
        self.zero = self.y_register == 0;
        self.negative = (self.y_register & 0b10000000) > 0;
    }


    pub fn execute(&mut self, cycles: i32, memory: &mut Memory) -> Result<i32, InstructionsError> {
        let cycles_requested = cycles as i32;
        let mut cycles = cycles;
        while cycles > 0 {
            let instruction_byte = Instruction::try_from(self.fetch_byte(&mut cycles, memory))?;
            match instruction_byte {
                Instruction::InsLdaIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsLdaZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value =
                        self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsLdaZpx => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let value =
                        self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsJsr => {
                    let subroutine_address = self.fetch_word(&mut cycles, memory);
                    memory.write_word(
                        self.program_counter - 1,
                        self.stack_pointer as Word + 0xFF,
                        &mut cycles,
                    );
                    self.stack_pointer += 1;
                    self.program_counter = subroutine_address;
                    cycles -= 1;
                    self.lda_set_status();
                }
                Instruction::InsLdaAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, absolute_address);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsLdaAbsX => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address + self.x_register as Word;

                    let value = self.read_byte(&mut cycles, memory, absolute_address_x);
                    self.a_register = value;
                    if absolute_address_x / 255 != absolute_address / 255 {
                        cycles -= 1;
                    }
                    self.lda_set_status();
                }
                Instruction::InsLdaAbsY => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address + self.y_register as Word;

                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    self.a_register = value;
                    if absolute_address_y / 255 != absolute_address / 255 {
                        cycles -= 1;
                    }
                    self.lda_set_status();
                }
                Instruction::InsLdaIndX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address_x = zero_page_address + self.x_register;
                    cycles -= 1;
                    let absolute_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address_x);

                    let value = self.read_byte(&mut cycles, memory, absolute_address);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsLdaIndY => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let absolute_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address);

                    let absolute_address_y = absolute_address + self.y_register as Word;

                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    self.a_register = value;
                    if absolute_address_y / 255 != absolute_address / 255 {
                        cycles -= 1;
                    }
                    self.lda_set_status();
                }
                Instruction::InsLdxIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.x_register = value;
                    self.ldx_set_status();
                }
                Instruction::InsLdxZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value =
                        self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.x_register = value;
                    self.ldx_set_status();
                }
                Instruction::InsLdxZpy => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.y_register);
                    cycles -= 1;
                    let value =
                        self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.x_register = value;
                    self.ldx_set_status();
                }
                Instruction::InsLdxAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, absolute_address);
                    self.x_register = value;
                    self.ldx_set_status();
                }
                Instruction::InsLdxAbsY => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address + self.y_register as Word;

                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    self.x_register = value;
                    if absolute_address_y / 255 != absolute_address / 255 {
                        cycles -= 1;
                    }
                    self.ldx_set_status();
                }
                Instruction::InsLdyIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.y_register = value;
                    self.ldy_set_status();
                }
                Instruction::InsLdyZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value =
                        self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.y_register = value;
                    self.ldy_set_status();
                }
                Instruction::InsLdyZpx => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let value =
                        self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.y_register = value;
                    self.ldy_set_status();
                }
                Instruction::InsLdyAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, absolute_address);
                    self.y_register = value;
                    self.ldy_set_status();
                }
                Instruction::InsLdyAbsX => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address + self.x_register as Word;

                    let value = self.read_byte(&mut cycles, memory, absolute_address_x);
                    self.y_register = value;
                    if absolute_address_x / 255 != absolute_address / 255 {
                        cycles -= 1;
                    }
                    self.ldy_set_status();
                }
                _ => {}
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

        let high_byte = (memory[self.program_counter] as Word) << 8;
        self.program_counter += 1;
        *cycles -= 1;

        let data: Word = low_byte | high_byte;
        data
    }

    pub fn read_byte(
        &self,
        cycles: &mut i32,
        memory: &mut Memory,
        address: Word,
    ) -> Byte {
        let data: Byte = memory[address];
        *cycles -= 1;
        data
    }

    pub fn read_word_from_zero_page(
        &self,
        cycles: &mut i32,
        memory: &mut Memory,
        address: Byte,
    ) -> Word {
        let low_byte = memory[address] as Word;
        *cycles -= 1;

        let high_byte = (memory[address + 1] as Word) << 8;
        *cycles -= 1;

        let data: Word = low_byte | high_byte;
        data
    }

    pub fn read_word_absolute(&self, cycles: &mut i32, memory: &mut Memory, address: Word) -> Word {
        let low_byte = memory[self.program_counter] as Word;
        *cycles -= 1;

        let high_byte = (memory[self.program_counter] as Word) << 8;
        *cycles -= 1;

        let data: Word = low_byte | high_byte;
        data
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionsError {
    InstructionDoesntExist(Byte),
}

pub enum Instruction {
    // LDA
    InsLdaIm = 0xA9,
    InsLdaZp = 0xA5,
    InsLdaZpx = 0xB5,
    InsLdaAbs = 0xAD,
    InsLdaAbsX = 0xBD,
    InsLdaAbsY = 0xB9,
    InsLdaIndX = 0xA1,
    InsLdaIndY = 0xB1,
    // LDX
    InsLdxIm = 0xA2,
    InsLdxZp = 0xA6,
    InsLdxZpy = 0xB6,
    InsLdxAbs = 0xAE,
    InsLdxAbsY = 0xBE,
    // LDY
    InsLdyIm = 0xA0,
    InsLdyZp = 0xA4,
    InsLdyZpx = 0xB4,
    InsLdyAbs = 0xAC,
    InsLdyAbsX = 0xBC,
    // Jumps
    InsJsr = 0x20,
    // STA
    InsStaZp = 0x85,
    InsStaZpx = 0x95,
    InsStaAbs = 0x8D,
    InsStaAbsX = 0x9D,
    InsStaAbsY = 0x99,
    InsStaIndX = 0x81,
    InsStaIndY = 0x91,
    // STX
    InsStxZp = 0x86,
    InsStxZpy = 0x96,
    InsStxAbs = 0x8E,
    // STY
    InsStyZp = 0x84,
    InsStyZpx = 0x94,
    InsStyAbs = 0x8C,
}

impl TryFrom<Byte> for Instruction {
    type Error = InstructionsError;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            // LDA
            0xA9 => Ok(Self::InsLdaIm),
            0xA5 => Ok(Self::InsLdaZp),
            0xB5 => Ok(Self::InsLdaZpx),
            0xAD => Ok(Self::InsLdaAbs),
            0xBD => Ok(Self::InsLdaAbsX),
            0xB9 => Ok(Self::InsLdaAbsY),
            0xA1 => Ok(Self::InsLdaIndX),
            0xB1 => Ok(Self::InsLdaIndY),
            // LDX
            0xA2 => Ok(Self::InsLdxIm),
            0xA6 => Ok(Self::InsLdxZp),
            0xB6 => Ok(Self::InsLdxZpy),
            0xAE => Ok(Self::InsLdxAbs),
            0xBE => Ok(Self::InsLdxAbsY),
            // LDY
            0xA0 => Ok(Self::InsLdyIm),
            0xA4 => Ok(Self::InsLdyZp),
            0xB4 => Ok(Self::InsLdyZpx),
            0xAC => Ok(Self::InsLdyAbs),
            0xBC => Ok(Self::InsLdyAbsX),
            // Jumps
            0x20 => Ok(Self::InsJsr),
            // STA
            0x85 => Ok(Self::InsStaZp),
            0x95 => Ok(Self::InsStaZpx),
            0x8D => Ok(Self::InsStaAbs),
            0x9D => Ok(Self::InsStaAbsX),
            0x99 => Ok(Self::InsStaAbsY),
            0x81 => Ok(Self::InsStaIndX),
            0x91 => Ok(Self::InsStaIndY),
            //  STX
            0x86 => Ok(Self::InsStxZp),
            0x96 => Ok(Self::InsStxZpy),
            0x8E => Ok(Self::InsStxAbs),
            // STY
            0x84 => Ok(Self::InsStyZp),
            0x95 => Ok(Self::InsStyZpx),
            0x8C => Ok(Self::InsStyAbs),
            _ => Err(InstructionsError::InstructionDoesntExist(value)),
        }
    }
}
