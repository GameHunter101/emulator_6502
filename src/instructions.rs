use crate::cpu::Byte;

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
    InsAndIm = 0x29,
    InsAndZp = 0x25,
    InsAndZpX = 0x35,
    InsAndAbs = 0x2D,
    InsAndAbsX = 0x3D,
    InsAndAbsY = 0x39,
    InsAndIndX = 0x21,
    InsAndIndY = 0x31,
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
    // BIT
    InsBitZp = 0x24,
    InsBitAbs = 0x2C,
    // Transfer
    InsTax = 0xAA,
    InsTay = 0xA8,
    InsTxa = 0x8A,
    InsTya = 0x98,
    // Increments
    InsInx = 0xE8,
    InsIny = 0xC8,
    // Decrements
    InsDex = 0xCA,
    InsDey = 0x88,
    // DEC
    InsDecZp = 0xC6,
    InsDecZpX = 0xD6,
    InsDecAbs = 0xCE,
    InsDecAbsX = 0xDE,
    // INC
    InsIncZp = 0xE6,
    InsIncZpX = 0xF6,
    InsIncAbs = 0xEE,
    InsIncAbsX = 0xFE,
    // Branch
    InsBeq = 0xF0,
    InsBne = 0xD0,
    InsBcs = 0xB0,
    InsBcc = 0x90,
    InsBmi = 0x30,
    InsBpl = 0x10,
    InsBvs = 0x70,
    InsBvc = 0x50,
    // Status flags
    InsClc = 0x18,
    InsSec = 0x38,
    InsCld = 0xD8,
    InsSed = 0xF8,
    InsCli = 0x58,
    InsSei = 0x78,
    InsClv = 0xB8,
    // Misc
    InsNop = 0xEA,
    // ADC
    InsAdcIm = 0x69,
    InsAdcZp = 0x65,
    InsAdcZpX = 0x75,
    InsAdcAbs = 0x6D,
    InsAdcAbsX = 0x7D,
    InsAdcAbsY = 0x79,
    InsAdcIndX = 0x61,
    InsAdcIndY = 0x71,
    // SBC
    InsSbcIm = 0xE9,
    InsSbcZp = 0xE5,
    InsSbcZpX = 0xF5,
    InsSbcAbs = 0xED,
    InsSbcAbsX = 0xFD,
    InsSbcAbsY = 0xF9,
    InsSbcIndX = 0xE1,
    InsSbcIndY = 0xF1,
    // CMP
    InsCmpIm = 0xC9,
    InsCmpZp = 0xC5,
    InsCmpZpX = 0xD5,
    InsCmpAbs = 0xCD,
    InsCmpAbsX = 0xDD,
    InsCmpAbsY = 0xD9,
    InsCmpIndX = 0xC1,
    InsCmpIndY = 0xD1,
    // CPX
    InsCpxIm = 0xE0,
    InsCpxZp = 0xE4,
    InsCpxAbs = 0xEC,
    // CPY
    InsCpyIm = 0xC0,
    InsCpyZp = 0xC4,
    InsCpyAbs = 0xCC,
    // ASL
    InsAslA = 0x0A,
    InsAslZp = 0x06,
    InsAslZpX = 0x16,
    InsAslAbs = 0x0E,
    InsAslAbsX = 0x1E,
    // LSR
    InsLsrA = 0x4A,
    InsLsrZp = 0x46,
    InsLsrZpX = 0x56,
    InsLsrAbs = 0x4E,
    InsLsrAbsX = 0x5E,
    // ROL
    InsRolA = 0x2A,
    InsRolZp = 0x26,
    InsRolZpX = 0x36,
    InsRolAbs = 0x2E,
    InsRolAbsX = 0x3E,
    // ROR
    InsRorA = 0x6A,
    InsRorZp = 0x66,
    InsRorZpX = 0x76,
    InsRorAbs = 0x6E,
    InsRorAbsX = 0x7E,
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
            0x29 => Ok(Self::InsAndIm),
            0x25 => Ok(Self::InsAndZp),
            0x35 => Ok(Self::InsAndZpX),
            0x2D => Ok(Self::InsAndAbs),
            0x3D => Ok(Self::InsAndAbsX),
            0x39 => Ok(Self::InsAndAbsY),
            0x21 => Ok(Self::InsAndIndX),
            0x31 => Ok(Self::InsAndIndY),
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
            // BIT
            0x24 => Ok(Self::InsBitZp),
            0x2C => Ok(Self::InsBitAbs),
            // Transfers
            0xAA => Ok(Self::InsTax),
            0xA8 => Ok(Self::InsTay),
            0x8A => Ok(Self::InsTxa),
            0x98 => Ok(Self::InsTya),
            // Increments
            0xE8 => Ok(Self::InsInx),
            0xC8 => Ok(Self::InsIny),
            // Decrements
            0xCA => Ok(Self::InsDex),
            0x88 => Ok(Self::InsDey),
            // DEC
            0xC6 => Ok(Self::InsDecZp),
            0xD6 => Ok(Self::InsDecZpX),
            0xCE => Ok(Self::InsDecAbs),
            0xDE => Ok(Self::InsDecAbsX),
            // INC
            0xE6 => Ok(Self::InsIncZp),
            0xF6 => Ok(Self::InsIncZpX),
            0xEE => Ok(Self::InsIncAbs),
            0xFE => Ok(Self::InsIncAbsX),
            // Branch
            0xF0 => Ok(Self::InsBeq),
            0xD0 => Ok(Self::InsBne),
            0xB0 => Ok(Self::InsBcs),
            0x90 => Ok(Self::InsBcc),
            0x30 => Ok(Self::InsBmi),
            0x10 => Ok(Self::InsBpl),
            0x70 => Ok(Self::InsBvs),
            0x50 => Ok(Self::InsBvc),
            // Status flags
            0x18 => Ok(Self::InsClc),
            0x38 => Ok(Self::InsSec),
            0xD8 => Ok(Self::InsCld),
            0xF8 => Ok(Self::InsSed),
            0x58 => Ok(Self::InsCli),
            0x78 => Ok(Self::InsSei),
            0xB8 => Ok(Self::InsClv),
            // Misc
            0xEA => Ok(Self::InsNop),
            // ADC
            0x69 => Ok(Self::InsAdcIm),
            0x65 => Ok(Self::InsAdcZp),
            0x75 => Ok(Self::InsAdcZpX),
            0x6D => Ok(Self::InsAdcAbs),
            0x7D => Ok(Self::InsAdcAbsX),
            0x79 => Ok(Self::InsAdcAbsY),
            0x61 => Ok(Self::InsAdcIndX),
            0x71 => Ok(Self::InsAdcIndY),
            // SBC
            0xE9 => Ok(Self::InsSbcIm),
            0xE5 => Ok(Self::InsSbcZp),
            0xF5 => Ok(Self::InsSbcZpX),
            0xED => Ok(Self::InsSbcAbs),
            0xFD => Ok(Self::InsSbcAbsX),
            0xF9 => Ok(Self::InsSbcAbsY),
            0xE1 => Ok(Self::InsSbcIndX),
            0xF1 => Ok(Self::InsSbcIndY),
            // CMP
            0xC9 => Ok(Self::InsCmpIm),
            0xC5 => Ok(Self::InsCmpZp),
            0xD5 => Ok(Self::InsCmpZpX),
            0xCD => Ok(Self::InsCmpAbs),
            0xDD => Ok(Self::InsCmpAbsX),
            0xD9 => Ok(Self::InsCmpAbsY),
            0xC1 => Ok(Self::InsCmpIndX),
            0xD1 => Ok(Self::InsCmpIndY),
            // CPX
            0xE0 => Ok(Self::InsCpxIm),
            0xE4 => Ok(Self::InsCpxZp),
            0xEC => Ok(Self::InsCpxAbs),
            // CPY
            0xC0 => Ok(Self::InsCpyIm),
            0xC4 => Ok(Self::InsCpyZp),
            0xCC => Ok(Self::InsCpyAbs),
            // ASL
            0x0A => Ok(Self::InsAslA),
            0x06 => Ok(Self::InsAslZp),
            0x16 => Ok(Self::InsAslZpX),
            0x0E => Ok(Self::InsAslAbs),
            0x1E => Ok(Self::InsAslAbsX),
            // LSR
            0x4A => Ok(Self::InsLsrA),
            0x46 => Ok(Self::InsLsrZp),
            0x56 => Ok(Self::InsLsrZpX),
            0x4E => Ok(Self::InsLsrAbs),
            0x5E => Ok(Self::InsLsrAbsX),
            // ROL
            0x2A => Ok(Self::InsRolA),
            0x26 => Ok(Self::InsRolZp),
            0x36 => Ok(Self::InsRolZpX),
            0x2E => Ok(Self::InsRolAbs),
            0x3E => Ok(Self::InsRolAbsX),
            // ROR
            0x6A => Ok(Self::InsRorA),
            0x66 => Ok(Self::InsRorZp),
            0x76 => Ok(Self::InsRorZpX),
            0x6E => Ok(Self::InsRorAbs),
            0x7E => Ok(Self::InsRorAbsX),
            _ => Err(InstructionsError::InstructionDoesntExist(value)),
        }
    }
}
