use crate::cpu::{Byte, Word, CPU};
use crate::memory::Memory;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, sys::Window};

#[derive(Debug)]
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

    pub fn get_pixels(&self) -> &[[Color;16];16]{
        &self.pixels
    }

    pub fn get_data(&mut self, data: Word) {
        let instruction = GraphicsInstruction::from(data);
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
            r: parsed_color_arr[0],
            g: parsed_color_arr[1],
            b: parsed_color_arr[2],
            a: 255,
        };

        match instruction.instruction_type {
            GraphicsInstructionType::SetClearColor => {
                self.clear_color = color;
            }
            GraphicsInstructionType::Draw => {
                self.pixels[parsed_coordinates[0] as usize][parsed_coordinates[1] as usize] = color;
            }
        }
        match instruction.clear_type {
            GraphicsClearType::Screen => self.pixels = [[self.clear_color; 16]; 16],
            GraphicsClearType::Pixel => {
                self.pixels[parsed_coordinates[0] as usize][parsed_coordinates[1] as usize] =
                    self.clear_color
            }
        }
    }
}

#[derive(Debug, Default)]
enum GraphicsInstructionType {
    #[default]
    SetClearColor,
    Draw,
}

#[derive(Debug, Default)]
enum GraphicsClearType {
    #[default]
    Screen,
    Pixel,
}

#[derive(Debug, Clone, Copy)]
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
        let bits: [bool; SIZE] = (0..SIZE)
            .map(|i| value & 2_u16.pow(i as u32) == 0)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        UNum(bits)
    }
}

impl<const SIZE: usize> Default for UNum<SIZE> {
    fn default() -> Self {
        Self([false; SIZE])
    }
}

#[derive(Debug, Default)]
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
            false => GraphicsInstructionType::SetClearColor,
        };
        let clear_type = match value & 0b0100000000000000 == 0b0100000000000000 {
            true => GraphicsClearType::Pixel,
            false => GraphicsClearType::Screen,
        };
        let coordinates: [UNum<4>; 2] = [UNum::from(value >> 10), UNum::from(value >> 6)];
        let color: [UNum<2>; 3] = [
            UNum::from(value),
            UNum::from(value >> 2),
            UNum::from(value >> 4),
        ];

        Self {
            instruction_type,
            clear_type,
            coordinates,
            color,
        }
    }
}
