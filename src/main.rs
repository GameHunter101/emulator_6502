#![allow(unused)]
use cpu::{Byte, Word, CPU};
use graphics_adapter::GraphicsAdapter;
use instructions::{Instruction, InstructionsError};
use memory::Memory;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, sys::Window};

pub mod cpu;
pub mod graphics_adapter;
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

    let graphics = GraphicsAdapter::new(Color {
        r: 100,
        g: 0,
        b: 0,
        a: 255,
    });

    let pixel_width = canvas.window().size().0 / 16;
    let pixel_height = canvas.window().size().1 / 16;

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

        for (row_index, row) in graphics.get_pixels().iter().enumerate() {
            for (col_index, pixel) in row.iter().enumerate() {
                canvas.set_draw_color(*pixel);
                canvas.fill_rect(Rect::new(
                    (pixel_width * col_index as u32) as i32,
                    (pixel_height * row_index as u32) as i32,
                    pixel_width,
                    pixel_height,
                ));
            }
        }

        canvas.present();
    }
}
