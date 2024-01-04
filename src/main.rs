mod cpu;
mod instr;
mod rom;
use cpu::{Chip8, NUM_COLS, NUM_ROWS};
use std::env;

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
fn emit_display_update(display: &[[u8; NUM_COLS as usize]; NUM_ROWS as usize]) {
    println!("Display updated!");
    pretty_print_display_grid(display);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading ROM into byte vec");
    let mut cpu = Chip8::new();
    let instrs = rom::read_rom(format!("./chip8-roms/programs/{}", file_path)); // Corrected line
    cpu.load_to_ram(&instrs);
    cpu.inspect_ram();
    cpu.cpu_loop(emit_display_update);
}
