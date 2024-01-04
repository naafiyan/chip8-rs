mod cpu;
mod instr;
mod rom;
use cpu::{Chip8, NUM_COLS, NUM_ROWS};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::time::Duration;

const RECT_HEIGHT: u32 = 10;
const RECT_WIDTH: u32 = 10;

fn draw_grid(canvas: &mut Canvas<Window>, display: &[[u8; NUM_COLS as usize]; NUM_ROWS as usize]) {
    for (i, row) in display.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let color = if cell == 0 {
                Color::RGB(0, 0, 0)
            } else {
                Color::RGB(255, 255, 255)
            };
            canvas.set_draw_color(color);
            let rect = Rect::new(j as i32 * 10, i as i32 * 10, RECT_WIDTH, RECT_HEIGHT);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn pretty_print_display_grid(display: &[[u8; NUM_COLS as usize]; NUM_ROWS as usize]) {
    println!("-----------------------------------");
    println!("DEBUG: Printing DISPLAY GRID");
    for row in display {
        for cell in row {
            let symbol = if cell.clone() == 1 { 'X' } else { ' ' };
            print!("{}", symbol);
        }
        println!();
    }
}
fn display_update(
    display: &[[u8; NUM_COLS as usize]; NUM_ROWS as usize],
    canvas: &mut Canvas<Window>,
) {
    println!("Display updated!");
    pretty_print_display_grid(display);
    draw_grid(canvas, display);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading ROM into byte vec");
    let mut cpu = Chip8::new();
    // load in rom file
    let instrs = rom::read_rom(format!("./chip8-roms/programs/{}", file_path));
    cpu.load_to_ram(&instrs);

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

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        cpu.cpu_loop(|display| display_update(display, &mut canvas));
    }
}
