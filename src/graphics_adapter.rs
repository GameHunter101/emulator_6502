use crate::cpu::{Byte, Word, CPU};
use crate::memory::Memory;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, sys::Window};

#[derive(Debug, Clone, Copy)]
pub struct GraphicsAdapter {
    pixels: [[Color; 16]; 16],
    clear_color: Color,
}

impl GraphicsAdapter {
    pub fn new(clear_color: Color) -> Self {
        Self {
            pixels: [[clear_color; 16]; 16],
            clear_color,
        }
    }

    pub fn get_pixels(&self) -> &[[Color; 16]; 16] {
        &self.pixels
    }

    pub fn get_data(&mut self, data: Word) {
        let instruction = GraphicsInstruction::from(data);
        self.execute_instruction(instruction);
    }

    fn execute_instruction(&mut self, instruction: GraphicsInstruction) {
        let coordinates = &instruction.coordinates;
        let parsed_coordinates: [Byte; 2] = [coordinates[0].into(), coordinates[1].into()];
        let color_bits = &instruction.color;
        let parsed_color_arr: [Byte; 3] = [
            color_bits[0].into(),
            color_bits[1].into(),
            color_bits[2].into(),
        ];
        let color = Color {
            r: parsed_color_arr[0] / 3 * 255,
            g: parsed_color_arr[1] / 3 * 255,
            b: parsed_color_arr[2] / 3 * 255,
            a: 255,
        };

        match instruction.instruction_type {
            GraphicsInstructionType::Clear => match instruction.clear_type {
                GraphicsClearType::Screen => self.pixels = [[self.clear_color; 16]; 16],
                GraphicsClearType::SetColor => {
                    self.clear_color = color;
                }
            },
            GraphicsInstructionType::Draw => {
                self.pixels[parsed_coordinates[0] as usize][parsed_coordinates[1] as usize] = color;
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum GraphicsInstructionType {
    #[default]
    Clear,
    Draw,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum GraphicsClearType {
    #[default]
    Screen,
    SetColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UNum<const SIZE: usize>([bool; SIZE]);

impl<const SIZE: usize> From<UNum<SIZE>> for Byte {
    fn from(val: UNum<SIZE>) -> Self {
        let mut num: Byte = 0;
        for (i, bit) in val.0.iter().enumerate() {
            num ^= (2_u8.pow(i as u32) * *bit as u8);
        }
        num
    }
}

impl<const SIZE: usize> From<Word> for UNum<SIZE> {
    fn from(value: Word) -> Self {
        let mut bits: [bool; SIZE] = (0..SIZE)
            .map(|i| value & 2_u16.pow(i as u32) != 0)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        bits.reverse();
        UNum(bits)
    }
}

impl<const SIZE: usize> Default for UNum<SIZE> {
    fn default() -> Self {
        Self([false; SIZE])
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct GraphicsInstruction {
    pub instruction_type: GraphicsInstructionType,
    pub clear_type: GraphicsClearType,
    pub coordinates: [UNum<4>; 2],
    pub color: [UNum<2>; 3],
}

impl From<Word> for GraphicsInstruction {
    fn from(value: Word) -> Self {
        let instruction_type = match value & 0b1000000000000000 == 0b1000000000000000 {
            true => GraphicsInstructionType::Draw,
            false => GraphicsInstructionType::Clear,
        };
        let clear_type = match value & 0b0100000000000000 == 0b0100000000000000 {
            true => GraphicsClearType::SetColor,
            false => GraphicsClearType::Screen,
        };
        let coordinates: [UNum<4>; 2] = [UNum::from(value >> 10), UNum::from(value >> 6)];
        // dbg!(value & 0b11)
        let color: [UNum<2>; 3] = [
            UNum::from(value >> 4),
            UNum::from(value >> 2),
            UNum::from(value),
        ];

        Self {
            instruction_type,
            clear_type,
            coordinates,
            color,
        }
    }
}

mod graphics_test {
    use crate::cpu::Word;

    use super::{GraphicsInstruction, UNum};

    #[test]
    fn test_unum_4() {
        let num: UNum<4> = UNum::from(1);

        assert_eq!(num, UNum([false, false, false, true]));
    }

    #[test]
    fn test_unum_2() {
        let num: UNum<2> = UNum::from(3);

        assert_eq!(num, UNum([true, true]));
    }

    #[test]
    fn test_instruction_parse() {
        let instruction: Word = 0b1000100010000011;
        let experimental = GraphicsInstruction::from(instruction);

        let theoretical = GraphicsInstruction {
            instruction_type: super::GraphicsInstructionType::Draw,
            clear_type: super::GraphicsClearType::Screen,
            coordinates: [UNum::from(2), UNum::from(2)],
            color: [UNum::from(0), UNum::from(0), UNum::from(3)],
        };

        assert_eq!(experimental.instruction_type, theoretical.instruction_type);
        assert_eq!(experimental.clear_type, theoretical.clear_type);
        assert_eq!(experimental.coordinates, theoretical.coordinates);
        assert_eq!(experimental.color, theoretical.color);
    }
}
