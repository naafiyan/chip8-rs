use std::cell::RefCell;
use std::rc::Rc;
use std::{thread::sleep, time::Duration};

use crate::display::{self};
use crate::{instr, key_input};

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
}

// load in Chip8 memory starting at address 0x200
impl Chip8<'_> {
    pub fn new<'a>(
        display: &'a mut display::Display,
        key_input: Rc<RefCell<key_input::KeyInput>>,
    ) -> Chip8<'a> {
        Chip8 {
            state: CPUState::new(),
            ram: [0; 4096],
            stack: Vec::new(),
            display,
            key_input,
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
        instr::op(curr_instr, self);
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
        println!("Loading instructions: {:?}", instrs);
        for instr in instrs {
            println!("curr instr: {:x?}", instr);
            self.ram[start] = instr.clone();
            println!("curr ram[{}]: {:x?}", start, self.ram[start]);
            start += 1
        }
    }

    // for debugging
    pub fn inspect_ram(&self) {
        println!("-----------------------------------");
        println!("DEBUG: Printing RAM contents");
        println!("{:?}", self.ram);
        for (i, mem) in self.ram.iter().enumerate() {
            println!("addr: {}, value: {:02x}", i, mem);
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

    pub fn skip_key(&mut self, reg_num: u8, skip: bool) {
        // determine if the key in reg_num is currently being held down
        let reg_key = self.get_reg(reg_num);
        let pressed_key = self.key_input.borrow().get_curr_pressed_key();

        // TODO: make this better/correct
        // if let Some(kc) = pressed_key {
        //     if let Some(hex_code) = key_input::chip8_keycode_map(kc) {
        //         if hex_code == reg_key && skip {
        //             self.incr_pc();
        //         } else if hex_code != reg_key && skip {
        //             self.incr_pc()
        //         }
        //     }
        // }
    }
}
