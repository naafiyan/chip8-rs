use std::cell::RefCell;
use std::rc::Rc;
use std::{thread::sleep, time::Duration};

use crate::display::{self};
use crate::emu_timer::EmuTimer;
use crate::{instr, key_input};

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// 700 instructions per second
// each instruction should then take 1000/700
const CLOCK_CYCLE: Duration = Duration::from_nanos(1430);
// 16 8-bit data registers named V0 to VF
struct CPUState {
    v_regs: [u8; 16], // data regs
    pc: u16,          // 12 bit register for address, always mask by &= 0xFFF
    i_reg: u16,
}

impl CPUState {
    fn new() -> CPUState {
        CPUState {
            v_regs: [0; 16],
            pc: 0x200, // 000 to 1FF = blocked off
            i_reg: 0,
        }
    }
}

pub struct Chip8<'a> {
    state: CPUState,
    ram: [u8; 4096], // 12 bit register for address, always mask by &= 0xFFF
    stack: Vec<u16>,
    pub display: &'a mut display::Display, // 32 rows, 64 columns, each can be 0 or 1 (on or off) - 0 is black, 1 is white
    key_input: Rc<RefCell<key_input::KeyInput>>,
    pub delay_timer: EmuTimer,
    pub sound_timer: EmuTimer,
}

// load in Chip8 memory starting at address 0x200
impl Chip8<'_> {
    pub fn new<'a>(
        display: &'a mut display::Display,
        key_input: Rc<RefCell<key_input::KeyInput>>,
    ) -> Chip8<'a> {
        // initialize the font system
        let mut ram = [0; 4096];
        ram[0x050..0x0A0].copy_from_slice(&FONT_SET);

        Chip8 {
            state: CPUState::new(),
            ram,
            stack: Vec::new(),
            display,
            key_input,
            delay_timer: EmuTimer::new(0),
            sound_timer: EmuTimer::new(0),
        }
    }

    pub fn cpu_loop(&mut self) {
        // redraw display during every loop if needed
        self.display.display_update();
        // fetch
        let curr_instr = {
            let curr_instr_1 = self.ram[self.state.pc as usize] as u16;
            let curr_instr_2 = self.ram[(self.state.pc + 1) as usize] as u16;
            (curr_instr_1 << 8) | curr_instr_2
        };
        self.state.pc += 2;

        // handle timers
        self.delay_timer.decr_time_left();
        self.sound_timer.decr_time_left();

        instr::op(curr_instr, self);

        // simulate Chip8 speed
        sleep(CLOCK_CYCLE);
    }

    pub fn stack_push(&mut self, addr: u16) {
        // TODO: account for limited stack space
        self.stack.push(addr)
    }

    pub fn stack_pop(&mut self) -> u16 {
        match self.stack.pop() {
            Some(addr) => addr,
            None => panic!("Error: Popping from an empty stack"),
        }
    }

    pub fn load_to_ram(&mut self, instrs: &[u8]) {
        // load all the instructions into memory starting at 0x200
        let mut start = 0x200;
        for instr in instrs {
            self.ram[start] = instr.clone();
            start += 1
        }
    }

    // for debugging
    pub fn inspect_ram(&self) {
        println!("-----------------------------------");
        println!("DEBUG: Printing RAM contents");
        for (i, mem) in self.ram.iter().enumerate() {
            println!("addr: {:03x}, value: {:04x}", i, mem);
        }
    }
    pub fn stack_push_pc(&mut self) {
        self.stack_push(self.state.pc);
    }

    pub fn set_pc(&mut self, addr: u16) {
        self.state.pc = addr;
    }

    pub fn incr_pc(&mut self) {
        self.state.pc += 2
    }

    pub fn decr_pc(&mut self) {
        self.state.pc -= 2
    }

    pub fn set_index_reg(&mut self, addr: u16) {
        self.state.i_reg = addr;
    }

    pub fn get_index_reg(&mut self) -> u16 {
        self.state.i_reg
    }

    pub fn set_reg(&mut self, reg_num: u8, val: u8) {
        self.state.v_regs[reg_num as usize] = val;
    }

    pub fn get_reg(&mut self, reg_num: u8) -> u8 {
        self.state.v_regs[reg_num as usize]
    }

    pub fn get_mem_data(&mut self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn skip_if_key(&mut self, reg_num: u8, is_same: bool) {
        // determine if the key in reg_num is currently being held down
        let reg_key = self.get_reg(reg_num);
        let pressed_key = {
            match self.key_input.borrow().get_curr_pressed_key() {
                // some key is being pressed down
                Some(kc) => {
                    if let Some(k) = key_input::chip8_keycode_map(kc) {
                        Some(k)
                    } else {
                        return;
                    }
                }
                // no key is currently pressed
                None => None,
            }
        };
        if let Some(k) = pressed_key {
            if k == reg_key && is_same {
                self.incr_pc()
            }
            if k != reg_key && !is_same {
                self.incr_pc()
            }
        } else {
            if !is_same {
                // no key => not same as key in VX
                self.incr_pc()
            }
        }
    }

    pub fn store_from_i(&mut self, vals: Vec<u8>) {
        // starting at I store values at increasing offsets
        let curr = self.get_index_reg() as usize;
        self.ram[curr..(curr + vals.len())].copy_from_slice(&vals);
    }

    pub fn load_from_i(&mut self, num_regs: u8) {
        let curr_index: u16 = self.get_index_reg();
        for i in 0..num_regs {
            let end = curr_index + (i as u16);
            self.state.v_regs[i as usize] = self.ram[end as usize];
        }
    }

    pub fn get_regs_in_range(&self, range: u8) -> Vec<u8> {
        let r = range as usize;
        self.state.v_regs[..r].to_vec()
    }

    pub fn block_till_key(&mut self, reg_num: u8) {
        let pressed_key = {
            match self.key_input.borrow().get_curr_pressed_key() {
                // some key is being pressed down
                Some(kc) => {
                    if let Some(k) = key_input::chip8_keycode_map(kc) {
                        Some(k)
                    } else {
                        return;
                    }
                }
                // no key is currently pressed
                None => None,
            }
        };
        if let Some(k) = pressed_key {
            self.set_reg(reg_num, k);
        } else {
            // store the pressed in reg
            self.decr_pc();
        }
    }

    pub fn load_char_into_index_reg(&mut self, val: u8) {
        let addr_of_char = 0x050 + (val * 5);
        println!("Addr of char: {:04x}", addr_of_char);
        self.set_index_reg(addr_of_char.into());
    }
}
