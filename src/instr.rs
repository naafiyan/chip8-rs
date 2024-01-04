use crate::cpu::{self, Chip8, NUM_COLS, NUM_ROWS};

fn first_nib(opcode: &u16) -> u16 {
    (opcode & 0xF000) >> 12
}
fn second_nib(opcode: &u16) -> u16 {
    (opcode & 0x0F00) >> 8
}
fn third_nib(opcode: &u16) -> u16 {
    (opcode & 0x00F0) >> 4
}
fn fourth_nib(opcode: &u16) -> u16 {
    opcode & 0x000F
}
fn first_byte(opcode: &u16) -> u16 {
    (opcode & 0xFF00) >> 8
}
fn second_byte(opcode: &u16) -> u16 {
    opcode & 0x00FF
}
fn addr_bits(opcode: &u16) -> u16 {
    opcode & 0x0FFF
}
// testing opcode helpers
#[test]
fn test_opcode_helpers() {
    let opcode = 0xABCD;
    assert_eq!(first_nib(&opcode), 0xA);
    assert_eq!(second_nib(&opcode), 0xB);
    assert_eq!(third_nib(&opcode), 0xC);
    assert_eq!(fourth_nib(&opcode), 0xD);
    assert_eq!(first_byte(&opcode), 0xAB);
    assert_eq!(second_byte(&opcode), 0xCD);
}

// 2 byte long integer in rust
pub fn op(opcode: u16, chip8: &mut Chip8) {
    // first nibble extracted by masking out last 3 nibbles
    // then bit shift by 12 (12 bits, i.e. 3 hex digits)
    let first_nibble = first_nib(&opcode);
    println!("opcode: {:x?}", opcode);
    println!("first_nib: {:x?}", first_nibble);
    match first_nibble {
        0x0 => op_0(opcode, chip8),
        0x1 => op_1(opcode, chip8),
        0x2 => op_2(opcode, chip8),
        0x3 => op_3(opcode, chip8),
        0x4 => op_4(opcode, chip8),
        0x5 => op_5(opcode, chip8),
        0x6 => op_6(opcode, chip8),
        0x7 => op_7(opcode, chip8),
        0x8 => op_8(opcode, chip8),
        0x9 => op_9(opcode, chip8),
        0xa => op_a(opcode, chip8),
        0xb => op_b(opcode, chip8),
        0xc => op_c(opcode, chip8),
        0xd => op_d(opcode, chip8),
        0xe => op_e(opcode, chip8),
        0xf => op_f(opcode, chip8),
        _ => println!("error: invalid opcode"),
    };
    println!("");
}

fn op_0(opcode: u16, chip8: &mut Chip8) {
    // extract the last 3 nibbles
    let instr = second_byte(&opcode);
    match instr {
        0xE0 => chip8.clear_display(),
        0xEE => println!("ee"),
        // TODO: better error handling
        _ => panic!("Error: Invalid opcode"),
    }
}

fn op_1(opcode: u16, chip8: &mut Chip8) {
    let addr = addr_bits(&opcode);
    chip8.jump(addr);
}

fn op_2(opcode: u16, chip8: &mut Chip8) {
    let addr = opcode & 0x0FFF;
    // TODO: call subroutine at addr
}

fn op_3(opcode: u16, chip8: &mut Chip8) {
    // e.g.for 0x3A43: 0x3A43 & 0x0F00 = 0x0A00
    // 0x0A00 >> 8 => 0x000A
    let reg_num = second_nib(&opcode);
    let nn = second_byte(&opcode);
    println!("op_3 second_nibble: {:X?}", reg_num);
    println!("op_3 constant: {:X?}", nn)
}

fn op_4(opcode: u16, chip8: &mut Chip8) {}
fn op_5(opcode: u16, chip8: &mut Chip8) {}
fn op_6(opcode: u16, chip8: &mut Chip8) {
    let reg_num = second_nib(&opcode);
    let val = second_byte(&opcode);
    chip8.set_reg(reg_num as u8, val as u8);
}
fn op_7(opcode: u16, chip8: &mut Chip8) {
    let reg_num = second_nib(&opcode);
    let val = second_byte(&opcode);
    let reg_val = chip8.get_reg(reg_num as u8);
    chip8.set_reg(reg_num as u8, (val as u8) + reg_val);
}
fn op_8(opcode: u16, chip8: &mut Chip8) {}
fn op_9(opcode: u16, chip8: &mut Chip8) {}
fn op_a(opcode: u16, chip8: &mut Chip8) {
    let addr = addr_bits(&opcode);
    chip8.set_index_reg(addr);
}
fn op_b(opcode: u16, chip8: &mut Chip8) {}
fn op_c(opcode: u16, chip8: &mut Chip8) {}
fn op_d(opcode: u16, chip8: &mut Chip8) {
    let x_reg = second_nib(&opcode);
    let y_reg = third_nib(&opcode);
    let x_start = chip8.get_reg(x_reg as u8) & (NUM_COLS - 1); // modulo 64
    let mut y = chip8.get_reg(y_reg as u8) & (NUM_ROWS - 1);
    chip8.set_reg(0xF, 0); // set VF to 0

    let i_reg = chip8.get_index_reg();
    println!("i_reg: {:06x}", i_reg);
    let n = fourth_nib(&opcode);
    for i in 0..n {
        if y == NUM_ROWS {
            break;
        }
        let sprite_data: u8 = chip8.get_mem_data(i_reg + i);
        println!("DEBUG: sprite_data: {:#06x}", sprite_data);
        // from most to least significant
        let mut x = x_start;
        for i in 0..8 {
            if x == NUM_COLS {
                break;
            }
            let curr_val = chip8.get_display(y, x);
            let curr_bit = (sprite_data >> (7 - i)) & 1;
            println!("DEBUG: curr_bit: {:x?}", curr_bit);

            // TODO: convert this to an easy XOR lol
            if (curr_val & curr_bit) > 0 {
                chip8.set_display(y, x, 0);
                chip8.set_reg(0xF, 1);
            } else if curr_bit > 0 && curr_val == 0 {
                chip8.set_display(y, x, 1);
            }
            x += 1;
        }
        y += 1;
    }
}
fn op_e(opcode: u16, chip8: &mut Chip8) {}
fn op_f(opcode: u16, chip8: &mut Chip8) {}
