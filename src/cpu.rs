use std::{thread::sleep, time::Duration};

use crate::instr;
pub const NUM_ROWS: u8 = 32;
pub const NUM_COLS: u8 = 64;

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

pub struct Chip8 {
    state: CPUState,
    ram: [u8; 4096], // 12 bit register for address, always mask by &= 0xFFF
    stack: Vec<u16>,
    display: [[u8; 64]; 32], // 32 rows, 64 columns, each can be 0 or 1 (on or off) - 0 is black, 1 is white
    display_updated: bool,
}

// load in Chip8 memory starting at address 0x200
impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            state: CPUState::new(),
            ram: [0; 4096],
            stack: Vec::new(),
            display: [[0; 64]; 32],
            display_updated: false,
        }
        // block off ram 000 to 1FF
    }

    pub fn cpu_loop<F>(&mut self, mut callback: F)
    where
        F: FnMut(&[[u8; NUM_COLS as usize]; NUM_ROWS as usize]),
    {
        if self.display_updated {
            callback(&self.display);
            self.display_updated = false;
        }
        // fetch
        let curr_instr = {
            let curr_instr_1 = self.ram[self.state.pc as usize] as u16;
            let curr_instr_2 = self.ram[(self.state.pc + 1) as usize] as u16;
            (curr_instr_1 << 8) | curr_instr_2
        };
        println!("current instr: {:x?}", curr_instr);
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

    pub fn clear_display(&mut self) {
        self.display = [[0; 64]; 32];
    }

    pub fn set_display(&mut self, row: u8, col: u8, val: u8) {
        self.display[row as usize][col as usize] = val;
        // self.pretty_print_display_grid();
        self.display_updated = true
    }

    pub fn get_display(&mut self, row: u8, col: u8) -> u8 {
        self.display[row as usize][col as usize]
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
}