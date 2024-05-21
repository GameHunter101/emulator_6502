#![allow(unused)]
use cpu::{Byte, Word, CPU};
use graphics_adapter::GraphicsAdapter;
use instructions::{Instruction, InstructionsError};
use memory::Memory;
use sdl2::{event::{Event, WindowEvent}, pixels::Color, rect::Rect, render::Canvas, sys::Window};

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

    let mut cpu = CPU::new_graphics(graphics, Some(0xFF00));
    let mut memory = Memory::initialize();

    let slice = &mut memory[0xFF00..0xFF0C];
    let path =
        std::path::Path::new(&std::env::current_dir().unwrap()).join("assembly\\test_code.prg");
    let file = std::fs::read(path).unwrap();
    slice.copy_from_slice(&file);

    let mut pixel_width = canvas.window().size().0 / 16;
    let mut pixel_height = canvas.window().size().1 / 16;

    let mut last_pixels = *cpu.get_graphics().unwrap().get_pixels();
    'running: loop {
        /* println!("{}", cpu);
        println!(
            "Val at stack: {:04x}\n\n",
            &memory[cpu.stack_pointer_to_address()]
        );
        println!(
            "Val at stack + 1: {:04x}\n\n",
            &memory[cpu.stack_pointer_to_address() + 1]
        ); */
        cpu.execute(1, &mut memory);

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
                Event::Window { win_event: WindowEvent::Resized(new_width, new_height), .. } => {
                    pixel_width = new_width as u32 / 16;
                    pixel_height = new_height as u32 / 16;
                }
                _ => {}
            }
        }

        let graphics = cpu.get_graphics().unwrap();

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

        last_pixels = *graphics.get_pixels();
    }
}

/* fn main() {
    let mut cpu = CPU::reset(None);
    let mut memory = Memory::initialize();

    cpu.program_counter = 0x400;

    let slice = &mut memory[0x000A..(0x000A + 65526)];
    let path = std::path::Path::new(&std::env::current_dir().unwrap())
        .join("assembly\\6502_functional_test.bin");
    dbg!(&path);
    let file = std::fs::read(path).unwrap();
    slice.copy_from_slice(&file);

    let mut last_print = std::time::Instant::now();
    loop {
        cpu.execute(1, &mut memory).unwrap();
        let now = std::time::Instant::now();
        if (now - last_print).as_millis() == 5 {
            println!("0x{:4x}", cpu.program_counter);
            last_print = now;
        }
        /* println!("{}", cpu);
        println!(
            "Val at stack: {:04x}\n\n",
            &memory[cpu.stack_pointer_to_address()]
        );
        println!(
            "Val at stack + 1: {:04x}\n\n",
            &memory[cpu.stack_pointer_to_address() + 1]
        ); */
        /* if cpu.program_counter == 0x3472 {
            break;
        } */
    }
} */
