#![allow(unused)]
use cpu::{Byte, Word, CPU};
use instructions::{Instruction, InstructionsError};
use memory::Memory;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, sys::Window};

pub mod cpu;
pub mod instructions;
pub mod memory;

#[cfg(test)]
mod tests {
    pub mod add_subtract_with_carry_tests;
    pub mod branch_tests;
    pub mod compare_register_tests;
    pub mod inc_dec_tests;
    pub mod jumps_and_calls_tests;
    pub mod load_tests;
    pub mod loading_program;
    pub mod logical_ops_tests;
    pub mod miscellaneous_tests;
    pub mod shifts_tests;
    pub mod stack_operations_tests;
    pub mod status_changes_tests;
    pub mod store_tests;
    pub mod transfer_register_tests;
}

fn main() {
    let context = sdl2::init().unwrap();
    let mut event_pump = context.event_pump().unwrap();
    let video = context.video().unwrap();

    let window = video
        .window("6502 Emulator", 400, 400)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    println!("Key pressed: {keycode}");
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(10, 10, 200, 200));

        canvas.present();
    }
}

#[derive(Debug)]
pub struct GraphicsAdapter {
    pixels: [[Color; 40]; 40],
}

impl GraphicsAdapter {
    pub fn get_data(&mut self, data: Word) {

    }
}
