use crate::{cpu::Chip8, utils};
use rand::Rng;

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

fn least_significant_bit(byte: &u8) -> u8 {
    byte & 1
}

fn most_significant_bit(byte: &u8) -> u8 {
    (byte & 0b10000000) >> 7
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

    let x = 0b00000001;
    assert_eq!(least_significant_bit(&x), 1);
    assert_eq!(most_significant_bit(&x), 0);
    let x = 0b10000001;
    assert_eq!(most_significant_bit(&x), 1);
}

// 2 byte long integer in rust
pub fn op(opcode: u16, chip8: &mut Chip8) {
    // first nibble extracted by masking out last 3 nibbles
    // then bit shift by 12 (12 bits, i.e. 3 hex digits)

    // -- DEBUG
    instr_code_to_name(&opcode);

    let first_nibble = first_nib(&opcode);
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
        0xE0 => chip8.display.clear_display(),
        0xEE => {
            // return
            let ret_addr = chip8.stack_pop();
            chip8.set_pc(ret_addr);
        }
        // TODO: better error handling
        _ => panic!("Error: Invalid opcode"),
    }
}

fn op_1(opcode: u16, chip8: &mut Chip8) {
    let addr = addr_bits(&opcode);
    chip8.set_pc(addr);
}

fn op_2(opcode: u16, chip8: &mut Chip8) {
    let addr = opcode & 0x0FFF;
    // push addr to call stack
    chip8.stack_push_pc();
    // jump
    chip8.set_pc(addr);
}

fn op_3(opcode: u16, chip8: &mut Chip8) {
    let reg_num = second_nib(&opcode) as u8;
    let nn = second_byte(&opcode) as u8;

    let val = chip8.get_reg(reg_num);
    if val == nn {
        chip8.incr_pc();
    }
}

fn op_4(opcode: u16, chip8: &mut Chip8) {
    let reg_num = second_nib(&opcode) as u8;
    let nn = second_byte(&opcode) as u8;

    let val = chip8.get_reg(reg_num);
    if val != nn {
        chip8.incr_pc();
    }
}

fn op_5(opcode: u16, chip8: &mut Chip8) {
    let reg_x = second_nib(&opcode) as u8;
    let reg_y = third_nib(&opcode) as u8;

    let x = chip8.get_reg(reg_x);
    let y = chip8.get_reg(reg_y);
    if x == y {
        chip8.incr_pc();
    }
}

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

fn op_8(opcode: u16, chip8: &mut Chip8) {
    let reg_x = second_nib(&opcode) as u8;
    let reg_y = third_nib(&opcode) as u8;
    let x = chip8.get_reg(reg_x);
    let y = chip8.get_reg(reg_y);

    match fourth_nib(&opcode) as u8 {
        0 => chip8.set_reg(reg_x, y),
        1 => chip8.set_reg(reg_x, x | y),
        2 => chip8.set_reg(reg_x, x & y),
        3 => chip8.set_reg(reg_x, x ^ y),
        4 => {
            let res = (x + y) as u16;
            if res > 255 {
                chip8.set_reg(reg_x, (res % 256) as u8);
                chip8.set_reg(0xF, 1);
            } else {
                chip8.set_reg(reg_x, res as u8);
                chip8.set_reg(0xF, 0);
            }
        }
        5 => {
            chip8.set_reg(0xF, 1);
            if x < y {
                // carry will occur
                chip8.set_reg(0xF, 0);
            }
            // ensure proper wrap
            chip8.set_reg(reg_x, x.wrapping_sub(y));
        }
        6 => {
            chip8.set_reg(0xF, least_significant_bit(&x));
            chip8.set_reg(reg_x, x >> 1);
        }
        7 => {
            chip8.set_reg(0xF, 1);
            if y < x {
                // carry will occur
                chip8.set_reg(0xF, 0);
            }
            // ensure proper wrap
            chip8.set_reg(reg_x, y.wrapping_sub(x));
        }
        0xe => {
            chip8.set_reg(0xF, most_significant_bit(&x));
            chip8.set_reg(reg_x, x << 1);
        }
        _ => panic!("Error: Invalid op8 type"),
    };

    // set reg_x to be reg_y
}
fn op_9(opcode: u16, chip8: &mut Chip8) {
    let reg_x = second_nib(&opcode) as u8;
    let reg_y = third_nib(&opcode) as u8;

    let x = chip8.get_reg(reg_x);
    let y = chip8.get_reg(reg_y);
    if x != y {
        chip8.incr_pc();
    }
}
fn op_a(opcode: u16, chip8: &mut Chip8) {
    let addr = addr_bits(&opcode);
    chip8.set_index_reg(addr);
}
fn op_b(opcode: u16, chip8: &mut Chip8) {
    let addr = addr_bits(&opcode);
    let v0 = chip8.get_reg(0x0) as u16;
    chip8.set_pc(v0 + addr);
}
fn op_c(opcode: u16, chip8: &mut Chip8) {
    // random number gen
    let r: u8 = rand::thread_rng().gen();
    let reg_x = second_nib(&opcode) as u8;
    let nn = second_byte(&opcode) as u8;
    chip8.set_reg(reg_x, r & nn);
}
fn op_d(opcode: u16, chip8: &mut Chip8) {
    let num_rows = chip8.display.num_rows as u8;
    let num_cols = chip8.display.num_cols as u8;

    let x_reg = second_nib(&opcode);
    let y_reg = third_nib(&opcode);
    let x_start = chip8.get_reg(x_reg as u8) & (num_cols - 1); // modulo 64
    let mut y = chip8.get_reg(y_reg as u8) & (num_rows - 1);
    chip8.set_reg(0xF, 0); // set VF to 0

    let i_reg = chip8.get_index_reg();
    let n = fourth_nib(&opcode);
    for i in 0..n {
        if y == num_rows {
            break;
        }
        let sprite_data: u8 = chip8.get_mem_data(i_reg + i);
        // from most to least significant
        let mut x = x_start;
        for i in 0..8 {
            if x == num_cols {
                break;
            }
            let curr_val = chip8.display.get_display_buffer(y, x);
            let curr_bit = (sprite_data >> (7 - i)) & 1;

            // XOR with carry flag
            if (curr_val & curr_bit) > 0 {
                chip8.display.set_display(y, x, 0);
                chip8.set_reg(0xF, 1);
            } else if curr_bit > 0 && curr_val == 0 {
                chip8.display.set_display(y, x, 1);
            }
            x += 1;
        }
        y += 1;
    }
}

// skip if key ops (delegate to the cpu)
fn op_e(opcode: u16, chip8: &mut Chip8) {
    let reg_num = second_nib(&opcode) as u8;
    match second_byte(&opcode) {
        0x9e => chip8.skip_if_key(reg_num, true),
        0xa1 => chip8.skip_if_key(reg_num, false),
        _ => panic!("Error: Invalid instruction"),
    }
}
fn op_f(opcode: u16, chip8: &mut Chip8) {
    let reg_num = second_nib(&opcode) as u8;
    let reg_val = chip8.get_reg(reg_num);
    match second_byte(&opcode) {
        // timers
        0x07 => {
            // set VX = delay_timer
            chip8.set_reg(reg_num, chip8.delay_timer.get_time_left() as u8);
        }
        0x15 => chip8.delay_timer.set_time_left(reg_val.into()),
        0x18 => chip8.sound_timer.set_time_left(reg_val.into()),

        // add to index
        0x1e => {
            let i_reg_val = chip8.get_index_reg();
            let res = i_reg_val + reg_val as u16;
            chip8.set_index_reg(res);
        }

        // block until key input
        0x0a => {
            chip8.block_till_key(reg_num);
        }

        // font char
        0x29 => {
            // index_reg I is set to address of hex char in VX
            chip8.load_char_into_index_reg(reg_num);
        }

        // binary-coded decimal conversion
        0x33 => {
            // takes reg_val and puts each digit into memory starting at I
            chip8.store_from_i(utils::extract_digits_u8(reg_val));
        }

        // store mem
        0x55 => {
            // get all register values
            let regs = chip8.get_regs_in_range(reg_num);
            let vals = regs.into_iter().map(|r_num| chip8.get_reg(r_num)).collect();
            chip8.store_from_i(vals);
        }
        // load mem
        0x65 => chip8.load_from_i(reg_num),
        _ => panic!("Error: Invalid instruction"),
    }
}

pub fn instr_code_to_name(opcode: &u16) {
    let first_nibble = first_nib(&opcode);
    let second_nib = second_nib(&opcode);
    let third_nib = third_nib(&opcode);
    let fourth_nib = fourth_nib(&opcode);

    let first_byte = first_byte(&opcode);
    let second_byte = second_byte(&opcode);
    let addr = addr_bits(&opcode);

    match first_nibble {
        0x0 => {
            if second_byte == 0xe0 {
                println!("00E0: Clear Screen");
            } else if second_byte == 0xee {
                println!("00EE: Return");
            }
        }
        0x1 => {
            println!("0{:03x}: Jump {:03x}", addr, addr);
        }
        0x2 => {
            println!("2{:03x}: Call {:03x}", addr, addr);
        }
        0x3 => {}
        0x4 => {}
        0x5 => {}
        0x6 => {}
        0x7 => {}
        0x8 => {}
        0x9 => {}
        0xa => {}
        0xb => {}
        0xc => {}
        0xd => {}
        0xe => {}
        0xf => {}
        _ => println!("error: invalid opcode"),
    };
}
