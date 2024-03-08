#![allow(unused)]
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

impl From<u8> for ProcessorFlags {
    fn from(value: u8) -> Self {
        ProcessorFlags {
            carry: value & 0b00000001 != 0,
            zero: value & 0b00000010 != 0,
            interupt_disable: value & 0b00000100 != 0,
            decimal_mode: value & 0b00001000 != 0,
            break_command: value & 0b00010000 != 0,
            overflow: value & 0b01000000 != 0,
            negative: value & 0b10000000 != 0,
        }
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
            result ^= 0b01000000;
        }
        if val.negative {
            result ^= 0b10000000;
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
        self.status.negative = (self.a_register & 0b10000000) > 0;
    }

    pub fn ldx_set_status(&mut self) {
        self.status.zero = self.x_register == 0;
        self.status.negative = (self.x_register & 0b10000000) > 0;
    }

    pub fn ldy_set_status(&mut self) {
        self.status.zero = self.y_register == 0;
        self.status.negative = (self.y_register & 0b10000000) > 0;
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
                _ => {}
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

        let high_byte = (memory[address + 1] as Word) << 8;
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

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionsError {
    InstructionDoesntExist(Byte),
}

pub enum Instruction {
    // LDA
    InsLdaIm = 0xA9,
    InsLdaZp = 0xA5,
    InsLdaZpX = 0xB5,
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
    InsLdyZpX = 0xB4,
    InsLdyAbs = 0xAC,
    InsLdyAbsX = 0xBC,
    // Jumps
    InsJsr = 0x20,
    InsRts = 0x60,
    InsJmpAbs = 0x4C,
    InsJmpInd = 0x6C,
    // STA
    InsStaZp = 0x85,
    InsStaZpX = 0x95,
    InsStaAbs = 0x8D,
    InsStaAbsX = 0x9D,
    InsStaAbsY = 0x99,
    InsStaIndX = 0x81,
    InsStaIndY = 0x91,
    // STX
    InsStxZp = 0x86,
    InsStxZpY = 0x96,
    InsStxAbs = 0x8E,
    // STY
    InsStyZp = 0x84,
    InsStyZpX = 0x94,
    InsStyAbs = 0x8C,
    // Transfer stack pointer
    InsTsx = 0xBA,
    InsTxs = 0x9A,
    InsPha = 0x48,
    InsPhp = 0x08,
    InsPla = 0x68,
    InsPlp = 0x28,
    // AND
    InsAndIm = 0x69,
    InsAndZp = 0x65,
    InsAndZpX = 0x75,
    InsAndAbs = 0x6D,
    InsAndAbsX = 0x7D,
    InsAndAbsY = 0x79,
    InsAndIndX = 0x61,
    InsAndIndY = 0x71,
    // EOR
    InsEorIm = 0x49,
    InsEorZp = 0x45,
    InsEorZpX = 0x55,
    InsEorAbs = 0x4D,
    InsEorAbsX = 0x5D,
    InsEorAbsY = 0x59,
    InsEorIndX = 0x41,
    InsEorIndY = 0x51,
    // ORA
    InsOraIm = 0x09,
    InsOraZp = 0x05,
    InsOraZpX = 0x15,
    InsOraAbs = 0x0D,
    InsOraAbsX = 0x1D,
    InsOraAbsY = 0x19,
    InsOraIndX = 0x01,
    InsOraIndY = 0x11,
}

impl TryFrom<Byte> for Instruction {
    type Error = InstructionsError;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            // LDA
            0xA9 => Ok(Self::InsLdaIm),
            0xA5 => Ok(Self::InsLdaZp),
            0xB5 => Ok(Self::InsLdaZpX),
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
            0xB4 => Ok(Self::InsLdyZpX),
            0xAC => Ok(Self::InsLdyAbs),
            0xBC => Ok(Self::InsLdyAbsX),
            // Jumps
            0x20 => Ok(Self::InsJsr),
            0x60 => Ok(Self::InsRts),
            0x4C => Ok(Self::InsJmpAbs),
            0x6C => Ok(Self::InsJmpInd),
            // STA
            0x85 => Ok(Self::InsStaZp),
            0x95 => Ok(Self::InsStaZpX),
            0x8D => Ok(Self::InsStaAbs),
            0x9D => Ok(Self::InsStaAbsX),
            0x99 => Ok(Self::InsStaAbsY),
            0x81 => Ok(Self::InsStaIndX),
            0x91 => Ok(Self::InsStaIndY),
            //  STX
            0x86 => Ok(Self::InsStxZp),
            0x96 => Ok(Self::InsStxZpY),
            0x8E => Ok(Self::InsStxAbs),
            // STY
            0x84 => Ok(Self::InsStyZp),
            0x94 => Ok(Self::InsStyZpX),
            0x8C => Ok(Self::InsStyAbs),
            // Transfer stack stack pointer
            0xBA => Ok(Self::InsTsx),
            0x9A => Ok(Self::InsTxs),
            0x48 => Ok(Self::InsPha),
            0x08 => Ok(Self::InsPhp),
            0x68 => Ok(Self::InsPla),
            0x28 => Ok(Self::InsPlp),
            // AND
            0x69 => Ok(Self::InsAndIm),
            0x65 => Ok(Self::InsAndZp),
            0x75 => Ok(Self::InsAndZpX),
            0x6D => Ok(Self::InsAndAbs),
            0x7D => Ok(Self::InsAndAbsX),
            0x79 => Ok(Self::InsAndAbsY),
            0x61 => Ok(Self::InsAndIndX),
            0x71 => Ok(Self::InsAndIndY),
            // EOR
            0x49 => Ok(Self::InsEorIm),
            0x45 => Ok(Self::InsEorZp),
            0x55 => Ok(Self::InsEorZpX),
            0x4D => Ok(Self::InsEorAbs),
            0x5D => Ok(Self::InsEorAbsX),
            0x59 => Ok(Self::InsEorAbsY),
            0x41 => Ok(Self::InsEorIndX),
            0x51 => Ok(Self::InsEorIndY),
            // ORA
            0x09 => Ok(Self::InsOraIm),
            0x05 => Ok(Self::InsOraZp),
            0x15 => Ok(Self::InsOraZpX),
            0x0D => Ok(Self::InsOraAbs),
            0x1D => Ok(Self::InsOraAbsX),
            0x19 => Ok(Self::InsOraAbsY),
            0x01 => Ok(Self::InsOraIndX),
            0x11 => Ok(Self::InsOraIndY),
            _ => Err(InstructionsError::InstructionDoesntExist(value)),
        }
    }
}
