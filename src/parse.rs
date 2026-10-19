use bstr::ByteSlice;
use bstr::ByteVec;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::instructions as ins;
use crate::memory;

macro_rules! unlock {
    () => {
        memory::MEM.lock().unwrap()
    };
}

pub fn read(path: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let contents: Vec<u8> = std::fs::read(path).expect("Uh oh!");
    let program: Vec<&[u8]> = contents.split_str("é").to_owned().collect();
    return program.into_iter().map(|instr| instr.to_vec()).collect();
}

pub fn parse_start() {
    parse(read("./program.txt"));
}

pub fn parse(str: Vec<Vec<u8>>) {
    if str.len() > 65536 || str.len() == 0 {
        println!(
            "bro ur program is too big, scale it back a little would ya?\neither that or you put in an empty file you stupid idiot"
        );
        return;
    }

    loop {
        let mem = unlock!();
        if mem.reg_pc as usize > str.len() - 1 {
            println!("PC went outside the bounds of the program.");
            return;
        }
        let cur = str[mem.reg_pc as usize].clone();
        std::mem::drop(mem);
        if parse_instruction(cur) {
            let mut mem = unlock!();
            mem.reg_pc += 1;
        }
    }
}

pub fn parse_instruction(instruct: Vec<u8>) -> bool {
    let instr = instruct.as_bytes();
    match instr[0] {
        b'A' => {
            ins::print_char_inplace(instr);
            return true;
        }
        b'B' => {
            ins::print_char_a();
            return true;
        }
        b'C' => {
            ins::load_immediate_a(instr);
            return true;
        }
        b'D' => {
            ins::jump(instr);
            return false;
        }
        b'E' => {
            ins::ret();
            return true;
        }
        b'Z' => {
            std::process::exit(0);
        }
        _ => {
            std::process::exit(1);
        }
    }
}
