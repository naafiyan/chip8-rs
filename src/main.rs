mod cpu;
mod display;
mod emu_timer;
mod instr;
mod key_input;
mod rom;
mod utils;

use cpu::Chip8;
use sdl2::event::Event;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::time::Duration;

// defaults for NUM_ROWS and NUM_COLS in the display grid
pub const NUM_ROWS: u8 = 32;
pub const NUM_COLS: u8 = 64;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = {
        match args.len() {
            1 => "./chip8-roms/programs/IBM Logo.ch8",
            2 => &args[1],
            _ => panic!("Error: Usage chip8-rs 'IBM Logo.ch8'"),
        }
    };

    let sdl_context = sdl2::init().unwrap();
    let vid_subsystem = sdl_context.video().unwrap();

    let window = vid_subsystem
        .window(
            "Chip8 Emulator",
            NUM_COLS as u32 * 10,
            NUM_ROWS as u32 * 10,
        )
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();

    // initialize the display
    let mut display = display::Display::new(NUM_ROWS as usize, NUM_COLS as usize, canvas);
    // TODO: is there a better way to associate key presses?
    let key_input = Rc::new(RefCell::new(key_input::KeyInput::new()));

    let mut cpu = Chip8::new(&mut display, Rc::clone(&key_input));
    // load in rom file
    let instrs = rom::read_rom(file_path.to_string());
    cpu.load_to_ram(&instrs);

    // -- DEBUG
    cpu.inspect_ram();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // TODO: technically this event loop can be done inside cpu
    // TODO: this leads into broader idea of refactoring cpu?
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                // update key_input's currently pressed key
                Event::KeyDown {
                    keycode: Some(kc), ..
                } => key_input.borrow_mut().update_curr_pressed_key(Some(kc)),
                // reset key being pressed
                Event::KeyUp { .. } => key_input.borrow_mut().update_curr_pressed_key(None),
                _ => {}
            }
        }
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        cpu.cpu_loop();
    }
}
