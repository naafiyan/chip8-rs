mod cpu;
mod display;
mod instr;
mod keyboard;
mod rom;

use cpu::Chip8;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::env;
use std::time::Duration;

use crate::display::Display;

// defaults for NUM_ROWS and NUM_COLS in the display grid
pub const NUM_ROWS: u8 = 32;
pub const NUM_COLS: u8 = 64;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading ROM into byte vec");

    let sdl_context = sdl2::init().unwrap();
    let vid_subsystem = sdl_context.video().unwrap();

    let window = vid_subsystem
        .window(
            "Chip8 Emulator",
            (NUM_COLS as u32 * 10) as u32,
            (NUM_ROWS as u32 * 10) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();

    // initialize the display
    let mut display = display::Display::new(NUM_ROWS as usize, NUM_COLS as usize, canvas);

    let mut cpu = Chip8::new(&mut display);
    // load in rom file
    let instrs = rom::read_rom(format!("./chip8-roms/programs/{}", file_path));
    cpu.load_to_ram(&instrs);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(kc), ..
                } => match keyboard::get_hex_val(kc) {
                    Some(hex_code) => {
                        println!("pressing {:x?}", hex_code)
                    }
                    None => println!("Other key pressed"),
                },
                _ => {}
            }
        }
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        cpu.cpu_loop();
    }
}
