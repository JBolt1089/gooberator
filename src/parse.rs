use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::memory as mem;

pub fn read(path: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    io::BufReader::new(File::open(path)?).lines().collect()
}

pub fn parse_start() {
    match read("./program.txt") {
        Ok(str) => parse(str),
        Err(e) => eprintln!("something went wrong: {e}"),
    }
}

pub fn parse(str: Vec<String>) {
    if str.len() > 65536 || str.len() == 0 {
        println!(
            "bro ur program is too big, scale it back a little would ya?\neither that or you put in an empty file you stupid idiot"
        );
        return;
    }

    loop {
        unsafe {
            if mem::REG_PC as usize > str.len() - 1 {
                println!("PC went outside the bounds of the program.");
                return;
            }
            let cur = str[mem::REG_PC as usize].clone();
            parse_instruction(cur);
        }
    }
}

pub fn parse_instruction(instruct: String) {
    let instr = instruct.into_bytes();
    match instr[0] {
        b'A' => println!("dis a test"),
        b'B' => std::process::exit(69),
        _ => std::process::exit(1),
    }
}
