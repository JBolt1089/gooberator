use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::instructions as ins;
use crate::memory;

/// Macro for unlocking the Mutex
macro_rules! unlock {
    () => {
        memory::MEM.lock().unwrap()
    };
}

/// Read file line by line and return into a vector of strings
pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(path)?).lines().collect()
}

/// Parse and repalce all the defs in the program vector with their associated value
pub fn parse_defs(lines: &Vec<String>, path: &String) -> io::Result<Vec<String>> {
    match read(path) {
        Ok(str) => {
            let mut result = lines.clone();
            for replacement in str {
                let rep: Vec<&str> = replacement.split_whitespace().collect();
                result = result.iter().map(|s| s.replace(rep[0], rep[1])).collect();
            }
            Ok(result)
        }
        Err(e) => {
            println!("Uh oh! {e}");
            std::process::exit(1);
        }
    }
}

/// Pre-loop parsing
pub fn parse_start(program_file: &String, refs_file: &String) {
    match read(program_file) {
        Ok(str) => match parse_defs(&str, refs_file) {
            Ok(str) => parse(str),
            Err(e) => println!("Uh oh! {e}"),
        },
        Err(e) => {
            println!("Uh oh! {e}");
        }
    }
}

/// Main parsing loop. parse_start() calls this function
pub fn parse(str: Vec<String>) {
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

/// Parse an instruction and call its respective function
pub fn parse_instruction(instruct: String) -> bool {
    let instr: Vec<&str> = instruct.split_whitespace().collect::<Vec<&str>>();
    match instr[0] {
        "prci" => {
            ins::print_char_immediate(instr);
            return true;
        }
        "prcr" => {
            ins::print_char_register(instr);
            return true;
        }
        "prab" => {
            ins::prab(instr);
            return true;
        }
        "li" => {
            ins::load_immediate(instr);
            return true;
        }
        "jmp" => {
            ins::jump(instr);
            return false;
        }
        "ret" => {
            ins::ret();
            return true;
        }
        "ldr" => {
            ins::ldr(instr);
            return true;
        }
        "str" => {
            ins::str(instr);
            return true;
        }
        "shl" => {
            ins::shl(instr);
            return true;
        }
        "shr" => {
            ins::shr(instr);
            return true;
        }
        "and" => {
            ins::and(instr);
            return true;
        }
        "or" => {
            ins::or(instr);
            return true;
        }
        "xor" => {
            ins::xor(instr);
            return true;
        }
        "strs" => {
            ins::strs(instr);
            return true;
        }
        "prns" => {
            ins::prns(instr);
            return true;
        }
        "inc" => {
            ins::inc(instr);
            return true;
        }
        "dec" => {
            ins::dec(instr);
            return true;
        }
        "add" => {
            ins::add(instr);
            return true;
        }
        "sub" => {
            ins::sub(instr);
            return true;
        }
        "jz" => {
            return ins::jz(instr);
        }
        "jnz" => {
            return ins::jnz(instr);
        }
        "end" => {
            std::process::exit(0);
        }
        "theendisnear" => {
            ins::report_error("🔔 the end is near"); // huh
            return true;
        }
        _ => {
            std::process::exit(1);
        }
    }
}
