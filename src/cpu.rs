#![allow(unused)]
use crate::instructions::{Instruction, InstructionsError};
use std::ops::BitOrAssign;

use crate::memory::Memory;

pub type Byte = u8;
pub type Word = u16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessorFlags {
    pub carry: bool,
    pub zero: bool,
    pub interupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl ProcessorFlags {
    pub const NEGATIVE_FLAG_BIT: Byte = 0b10000000;
    pub const OVERFLOW_FLAG_BIT: Byte = 0b01000000;
    pub fn into_u8(&self) -> u8 {
        let mut value = 0;
        value |= self.carry as u8;
        value |= (self.zero as u8) << 1;
        value |= (self.interupt_disable as u8) << 2;
        value |= (self.decimal_mode as u8) << 3;
        value |= (self.break_command as u8) << 4;
        value |= (self.overflow as u8) << 6;
        value |= (self.negative as u8) << 7;

        value
    }
}

impl From<u8> for ProcessorFlags {
    fn from(value: u8) -> Self {
        ProcessorFlags {
            carry: value & 0b00000001 != 0,
            zero: value & 0b00000010 != 0,
            interupt_disable: value & 0b00000100 != 0,
            decimal_mode: value & 0b00001000 != 0,
            break_command: value & 0b00010000 != 0,
            overflow: value & Self::OVERFLOW_FLAG_BIT != 0,
            negative: value & Self::NEGATIVE_FLAG_BIT != 0,
        }
    }
}

impl BitOrAssign for ProcessorFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        let as_u8 = self.into_u8();
        *self = Self::from(as_u8 | rhs.into_u8());
    }
}

impl From<ProcessorFlags> for u8 {
    fn from(val: ProcessorFlags) -> Self {
        let mut result = 0;
        if val.carry {
            result ^= 0b00000001;
        }
        if val.zero {
            result ^= 0b00000010;
        }
        if val.interupt_disable {
            result ^= 0b00000100;
        }
        if val.decimal_mode {
            result ^= 0b00001000;
        }
        if val.break_command {
            result ^= 0b00010000;
        }
        if val.overflow {
            result ^= ProcessorFlags::OVERFLOW_FLAG_BIT;
        }
        if val.negative {
            result ^= ProcessorFlags::NEGATIVE_FLAG_BIT;
        }
        result
    }
}

impl Default for ProcessorFlags {
    fn default() -> Self {
        ProcessorFlags {
            carry: false,
            zero: false,
            interupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }
}

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
    pub status: ProcessorFlags,
}

impl CPU {
    pub fn reset(reset_vector: Option<Word>) -> CPU {
        let program_counter = match reset_vector {
            Some(address) => address,
            None => 0xFFFC,
        };
        CPU {
            program_counter,
            stack_pointer: 0xFF,
            a_register: 0,
            x_register: 0,
            y_register: 0,
            status: ProcessorFlags::default(),
        }
    }

    pub fn lda_set_status(&mut self) {
        self.status.zero = self.a_register == 0;
        self.status.negative = (self.a_register & ProcessorFlags::NEGATIVE_FLAG_BIT) > 0;
    }

    pub fn ldx_set_status(&mut self) {
        self.status.zero = self.x_register == 0;
        self.status.negative = (self.x_register & ProcessorFlags::NEGATIVE_FLAG_BIT) > 0;
    }

    pub fn ldy_set_status(&mut self) {
        self.status.zero = self.y_register == 0;
        self.status.negative = (self.y_register & ProcessorFlags::NEGATIVE_FLAG_BIT) > 0;
    }

    pub fn execute(&mut self, cycles: i32, memory: &mut Memory) -> Result<i32, InstructionsError> {
        let cycles_requested = cycles as i32;
        let mut cycles = cycles;
        while cycles > 0 {
            let instruction_byte = Instruction::try_from(self.fetch_byte(&mut cycles, memory))?;
            match instruction_byte {
                // LDA
                Instruction::InsLdaIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsLdaZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register = value;
                    self.lda_set_status();
                }
                Instruction::InsLdaZpX => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register = value;
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
                    if !CPU::check_same_page(absolute_address, absolute_address_x) {
                        cycles -= 1;
                    }
                    self.lda_set_status();
                }
                Instruction::InsLdaAbsY => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address + self.y_register as Word;

                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    self.a_register = value;
                    if !CPU::check_same_page(absolute_address, absolute_address_y) {
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
                    if !CPU::check_same_page(absolute_address, absolute_address_y) {
                        cycles -= 1;
                    }
                    self.lda_set_status();
                }
                // LDX
                Instruction::InsLdxIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.x_register = value;
                    self.ldx_set_status();
                }
                Instruction::InsLdxZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.x_register = value;
                    self.ldx_set_status();
                }
                Instruction::InsLdxZpy => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.y_register);
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
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
                    if !CPU::check_same_page(absolute_address, absolute_address_y) {
                        cycles -= 1;
                    }
                    self.ldx_set_status();
                }
                // LDY
                Instruction::InsLdyIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.y_register = value;
                    self.ldy_set_status();
                }
                Instruction::InsLdyZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.y_register = value;
                    self.ldy_set_status();
                }
                Instruction::InsLdyZpX => {
                    let mut zero_page_address = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
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
                    if !CPU::check_same_page(absolute_address, absolute_address_x) {
                        cycles -= 1;
                    }
                    self.ldy_set_status();
                }
                // Jumps
                Instruction::InsJsr => {
                    let subroutine_address = self.fetch_word(&mut cycles, memory);
                    self.push_program_counter_to_stack(&mut cycles, memory);
                    self.program_counter = subroutine_address;
                }
                Instruction::InsRts => {
                    let return_address = self.pop_word_from_stack(&mut cycles, memory);
                    self.program_counter = return_address + 1;
                    cycles -= 2;
                }
                Instruction::InsJmpAbs => {
                    let address = self.fetch_word(&mut cycles, memory);
                    self.program_counter = address;
                }
                Instruction::InsJmpInd => {
                    let indirect_address = self.fetch_word(&mut cycles, memory);
                    let address = self.read_word_absolute(&mut cycles, memory, indirect_address);
                    self.program_counter = address;
                }
                // STA
                Instruction::InsStaZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    self.write_byte(
                        self.a_register,
                        zero_page_address as Word,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStaZpX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory) + self.x_register;
                    cycles -= 1;
                    self.write_byte(
                        self.a_register,
                        zero_page_address as Word,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStaAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    self.write_byte(self.a_register, absolute_address, &mut cycles, memory);
                }
                Instruction::InsStaAbsX => {
                    let absolute_address =
                        self.fetch_word(&mut cycles, memory) + self.x_register as Word;
                    cycles -= 1;
                    self.write_byte(self.a_register, absolute_address, &mut cycles, memory);
                }
                Instruction::InsStaAbsY => {
                    let absolute_address =
                        self.fetch_word(&mut cycles, memory) + self.y_register as Word;
                    cycles -= 1;
                    self.write_byte(self.a_register, absolute_address, &mut cycles, memory);
                }
                Instruction::InsStaIndX => {
                    let zero_page_address = self
                        .fetch_byte(&mut cycles, memory)
                        .wrapping_add(self.x_register);
                    cycles -= 1;
                    let indexed_indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address);
                    self.write_byte(
                        self.a_register,
                        indexed_indirect_address,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStaIndY => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let indirect_indexed_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address)
                            + self.y_register as Word;
                    cycles -= 1;
                    self.write_byte(
                        self.a_register,
                        indirect_indexed_address,
                        &mut cycles,
                        memory,
                    );
                }
                // STX
                Instruction::InsStxZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    self.write_byte(
                        self.x_register,
                        zero_page_address as Word,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStxZpY => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory) + self.y_register;
                    cycles -= 1;
                    self.write_byte(
                        self.x_register,
                        zero_page_address as Word,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStxAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    self.write_byte(self.x_register, absolute_address, &mut cycles, memory);
                }
                // STY
                Instruction::InsStyZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    self.write_byte(
                        self.y_register,
                        zero_page_address as Word,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStyZpX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory) + self.x_register;
                    cycles -= 1;
                    self.write_byte(
                        self.y_register,
                        zero_page_address as Word,
                        &mut cycles,
                        memory,
                    );
                }
                Instruction::InsStyAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    self.write_byte(self.y_register, absolute_address, &mut cycles, memory);
                }
                Instruction::InsTsx => {
                    self.x_register = self.stack_pointer;
                    cycles -= 1;
                    self.ldx_set_status();
                }
                Instruction::InsTxs => {
                    self.stack_pointer = self.x_register;
                    cycles -= 1;
                }
                Instruction::InsPha => {
                    self.push_byte_to_stack(self.a_register, &mut cycles, memory);
                }
                Instruction::InsPla => {
                    self.a_register = self.pop_byte_from_stack(&mut cycles, memory);
                    self.lda_set_status();
                }
                Instruction::InsPhp => {
                    self.push_byte_to_stack(self.status.into(), &mut cycles, memory);
                }
                Instruction::InsPlp => {
                    self.status = self.pop_byte_from_stack(&mut cycles, memory).into();
                }
                // AND
                Instruction::InsAndIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndZpX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory) + self.x_register;
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndAbs => {
                    let address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, address);
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndAbsX => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address + self.x_register as Word;
                    let value = self.read_byte(&mut cycles, memory, absolute_address_x);
                    if !CPU::check_same_page(absolute_address, absolute_address_x) {
                        cycles -= 1;
                    }
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndAbsY => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address + self.y_register as Word;
                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    if !CPU::check_same_page(absolute_address, absolute_address_y) {
                        cycles -= 1;
                    }
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndIndX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address_x = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address_x);
                    let value = self.read_byte(&mut cycles, memory, indirect_address);
                    self.a_register &= value;
                    self.lda_set_status();
                }
                Instruction::InsAndIndY => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address);
                    let indirect_address_y = indirect_address + self.y_register as Word;
                    if !CPU::check_same_page(indirect_address, indirect_address_y) {
                        cycles -= 1;
                    }
                    let value = self.read_byte(&mut cycles, memory, indirect_address_y);
                    self.a_register &= value;
                    self.lda_set_status();
                }
                // EOR
                Instruction::InsEorIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorZpX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory) + self.x_register;
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorAbs => {
                    let address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, address);
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorAbsX => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address + self.x_register as Word;
                    let value = self.read_byte(&mut cycles, memory, absolute_address_x);
                    if !CPU::check_same_page(absolute_address, absolute_address_x) {
                        cycles -= 1;
                    }
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorAbsY => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address + self.y_register as Word;
                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    if !CPU::check_same_page(absolute_address, absolute_address_y) {
                        cycles -= 1;
                    }
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorIndX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address_x = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address_x);
                    let value = self.read_byte(&mut cycles, memory, indirect_address);
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                Instruction::InsEorIndY => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address);
                    let indirect_address_y = indirect_address + self.y_register as Word;
                    if !CPU::check_same_page(indirect_address, indirect_address_y) {
                        cycles -= 1;
                    }
                    let value = self.read_byte(&mut cycles, memory, indirect_address_y);
                    self.a_register ^= value;
                    self.lda_set_status();
                }
                // ORA
                Instruction::InsOraIm => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraZpX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory) + self.x_register;
                    cycles -= 1;
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraAbs => {
                    let address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, address);
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraAbsX => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_x = absolute_address + self.x_register as Word;
                    let value = self.read_byte(&mut cycles, memory, absolute_address_x);
                    if !CPU::check_same_page(absolute_address, absolute_address_x) {
                        cycles -= 1;
                    }
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraAbsY => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let absolute_address_y = absolute_address + self.y_register as Word;
                    let value = self.read_byte(&mut cycles, memory, absolute_address_y);
                    if !CPU::check_same_page(absolute_address, absolute_address_y) {
                        cycles -= 1;
                    }
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraIndX => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let zero_page_address_x = zero_page_address.wrapping_add(self.x_register);
                    cycles -= 1;
                    let indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address_x);
                    let value = self.read_byte(&mut cycles, memory, indirect_address);
                    self.a_register |= value;
                    self.lda_set_status();
                }
                Instruction::InsOraIndY => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let indirect_address =
                        self.read_word_from_zero_page(&mut cycles, memory, zero_page_address);
                    let indirect_address_y = indirect_address + self.y_register as Word;
                    if !CPU::check_same_page(indirect_address, indirect_address_y) {
                        cycles -= 1;
                    }
                    let value = self.read_byte(&mut cycles, memory, indirect_address_y);
                    self.a_register |= value;
                    self.lda_set_status();
                }
                // BIT
                Instruction::InsBitZp => {
                    let zero_page_address = self.fetch_byte(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, zero_page_address as Word);
                    self.status.zero = (self.a_register & value) == 0;
                    self.status.negative = (value & ProcessorFlags::NEGATIVE_FLAG_BIT) != 0;
                    self.status.overflow = (value & ProcessorFlags::OVERFLOW_FLAG_BIT) != 0;
                }
                Instruction::InsBitAbs => {
                    let absolute_address = self.fetch_word(&mut cycles, memory);
                    let value = self.read_byte(&mut cycles, memory, absolute_address);
                    self.status.zero = (self.a_register & value) == 0;
                    self.status.negative = (value & ProcessorFlags::NEGATIVE_FLAG_BIT) != 0;
                    self.status.overflow = (value & ProcessorFlags::OVERFLOW_FLAG_BIT) != 0;
                }
                _ => {
                    break;
                }
            }
        }
        Ok(cycles_requested - cycles)
    }

    pub fn check_same_page(address_a: Word, address_b: Word) -> bool {
        address_a / 256 == address_b / 256
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

    pub fn read_byte(&self, cycles: &mut i32, memory: &mut Memory, address: Word) -> Byte {
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

        let high_byte = (memory[address.wrapping_add(1)] as Word) << 8;
        *cycles -= 1;

        let data: Word = low_byte | high_byte;
        data
    }

    pub fn read_word_absolute(&self, cycles: &mut i32, memory: &mut Memory, address: Word) -> Word {
        let low_byte = memory[address] as Word;
        *cycles -= 1;

        let high_byte = (memory[address + 1] as Word) << 8;
        *cycles -= 1;

        let data: Word = low_byte | high_byte;
        data
    }

    pub fn write_word(&mut self, data: Word, address: Word, cycles: &mut i32, memory: &mut Memory) {
        let data_bytes = data.to_le_bytes();
        memory[address] = data_bytes[0];
        *cycles -= 1;
        memory[address + 1] = data_bytes[1];
        *cycles -= 1;
    }

    pub fn write_byte(&mut self, data: Byte, address: Word, cycles: &mut i32, memory: &mut Memory) {
        memory[address] = data;
        *cycles -= 1;
    }

    pub fn stack_pointer_to_address(&self) -> Word {
        0x100 | self.stack_pointer as Word
    }

    pub fn push_program_counter_to_stack(&mut self, cycles: &mut i32, memory: &mut Memory) {
        self.write_word(
            self.program_counter - 1,
            self.stack_pointer_to_address() - 1,
            cycles,
            memory,
        );
        self.stack_pointer -= 2;
        *cycles -= 1;
    }

    pub fn pop_word_from_stack(&mut self, cycles: &mut i32, memory: &mut Memory) -> Word {
        let return_address =
            self.read_word_absolute(cycles, memory, self.stack_pointer_to_address() + 1);
        self.stack_pointer += 2;
        *cycles -= 1;
        return_address
    }

    pub fn push_byte_to_stack(&mut self, data: Byte, cycles: &mut i32, memory: &mut Memory) {
        memory[self.stack_pointer_to_address()] = data;
        *cycles -= 1;
        self.stack_pointer -= 1;
        *cycles -= 1;
    }

    pub fn pop_byte_from_stack(&mut self, cycles: &mut i32, memory: &mut Memory) -> Byte {
        self.stack_pointer += 1;
        let data = memory[self.stack_pointer_to_address()];
        *cycles -= 3;
        data
    }
}
