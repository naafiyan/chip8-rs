use std::fs::File;
use std::io::{Read};

pub fn read_rom(file_path: String) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Error opening file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");

    let mut instrs = Vec::new();
    for byte in buffer {
        instrs.push(byte);
    }
    instrs
}
