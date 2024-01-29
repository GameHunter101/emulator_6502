pub type Byte = u8;
pub type Word = u16;

#[derive(Debug)]
pub struct CPU {
    // Addresses
    program_counter: Word,
    stack_pointer: Word,

    // Registers
    a_register: Byte,
    x_register: Byte,
    y_register: Byte,

    // Status flags
    carry: bool,
    zero: bool,
    interupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow: bool,
    negative: bool,
}

impl CPU {
    pub fn reset() -> CPU {
        CPU {
            program_counter: 0xFFFC,
            stack_pointer: 0x0100,
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
}
