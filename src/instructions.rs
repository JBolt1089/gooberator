use std::process::exit;

use crate::memory;

macro_rules! lock {
    () => {
        memory::MEM.lock().unwrap()
    };
}

fn report_error(str: &str) {
    let mem = lock!();
    println!(
        "Error: {}\nRegisters: {:?}",
        str,
        (mem.reg_pc, mem.reg_a, mem.reg_b, mem.reg_c)
    );
    exit(1)
}

pub fn print_char_inplace(instr: &[u8]) {
    if instr.len() != 2 {
        report_error("wrong number of arguments given");
    }
    print!("{}", instr[1] as char);
}

pub fn print_char_a() {
    let mem = lock!();

    print!("{}", (mem.reg_a as u8) as char);
}

pub fn load_immediate_a(instr: &[u8]) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    mem.reg_a = u16::from_be_bytes([instr[1], instr[2]]);
}

pub fn jump(instr: &[u8]) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();
    let pc = mem.reg_pc;
    mem.stack.push(pc);
    mem.reg_pc = u16::from_be_bytes([instr[1], instr[2]]);
}

pub fn ret() {
    let mut mem = lock!();

    let ret_to = mem.stack.pop();
    match ret_to {
        Some(addr) => mem.reg_pc = addr,
        None => report_error("attempted to return with nothing on the stack"),
    }
}
