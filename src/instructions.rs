use std::collections::VecDeque;
use std::process::exit;

use crate::memory;

macro_rules! lock {
    () => {
        memory::MEM.lock().unwrap()
    };
}

pub fn report_error(str: &str) {
    let mem = lock!();
    println!(
        "Error: {}\nRegisters: {:?}",
        str,
        (mem.reg_pc, mem.reg_a, mem.reg_b, mem.reg_c)
    );
    exit(1)
}

fn parse_hex_arg(str: &str) -> u16 {
    match str.strip_prefix("0x") {
        Some(x) => match u16::from_str_radix(x, 16) {
            Ok(y) => {
                return y;
            }
            Err(_) => {
                report_error("error parsing hex");
                return 69;
            }
        },
        None => match u16::from_str_radix(str, 16) {
            Ok(y) => {
                return y;
            }
            Err(_) => {
                report_error("error parsing hex");
                return 69;
            }
        },
    }
}

pub fn print_char_immediate(instr: Vec<&str>) {
    if instr.len() != 2 {
        report_error("wrong number of arguments given");
    }

    let c: char = parse_hex_arg(instr[1]) as u8 as char;
    print!("{c}");
}

pub fn print_char_register(instr: Vec<&str>) {
    let mem = lock!();

    print!("{}", (mem.clone().get_reg_str(instr[1]) as u8) as char);
}

pub fn load_immediate(instr: Vec<&str>) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    mem.update_reg_str(instr[1], parse_hex_arg(instr[2]));
}

pub fn jump(instr: Vec<&str>) {
    if instr.len() != 2 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();
    let pc = mem.reg_pc;
    mem.stack.push(pc);
    mem.reg_pc = parse_hex_arg(instr[1]);
}

pub fn ret() {
    let mut mem = lock!();

    let ret_to = mem.stack.pop();
    match ret_to {
        Some(addr) => mem.reg_pc = addr,
        None => report_error("attempted to return with nothing on the stack"),
    }
}

pub fn ldr(instr: Vec<&str>) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }
    let mut mem = lock!();
    let num = mem.memory[parse_hex_arg(instr[2]) as usize];
    mem.update_reg_str(instr[1], num);
}

pub fn str(instr: Vec<&str>) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }
    let mut mem = lock!();

    let num = mem.clone().get_reg_str(instr[1]);
    mem.memory[parse_hex_arg(instr[2]) as usize] = num;
}

pub fn shl(instr: Vec<&str>) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let mut num = mem.clone().get_reg_str(instr[1]);
    num <<= parse_hex_arg(instr[2]);
    mem.update_reg_str(instr[1], num);
}

pub fn shr(instr: Vec<&str>) {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let mut num = mem.clone().get_reg_str(instr[1]);
    num >>= parse_hex_arg(instr[2]);
    mem.update_reg_str(instr[1], num);
}

pub fn and(instr: Vec<&str>) {
    if instr.len() != 4 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let input1 = mem.clone().get_reg_str(instr[1]);
    let input2 = mem.clone().get_reg_str(instr[2]);
    let output = input1 & input2;

    mem.update_reg_str(instr[3], output);
}

pub fn or(instr: Vec<&str>) {
    if instr.len() != 4 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let input1 = mem.clone().get_reg_str(instr[1]);
    let input2 = mem.clone().get_reg_str(instr[2]);
    let output = input1 | input2;

    mem.update_reg_str(instr[3], output);
}

pub fn xor(instr: Vec<&str>) {
    if instr.len() != 4 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let input1 = mem.clone().get_reg_str(instr[1]);
    let input2 = mem.clone().get_reg_str(instr[2]);
    let output = input1 ^ input2;

    mem.update_reg_str(instr[3], output);
}

pub fn prab(instr: Vec<&str>) {
    if instr.len() != 2 {
        report_error("wrong number of arguments given");
    }

    let mem = lock!();

    println!("\n{:#b}", mem.clone().get_reg_str(instr[1]));
}

pub fn strs(instr: Vec<&str>) {
    if instr.len() < 2 {
        report_error("wrong number of arguments given");
    }

    let mut args = VecDeque::from(instr);
    let mut mem = lock!();

    let mut target = parse_hex_arg(args[1]);
    args.pop_front();

    while args.len() > 0 {
        let char = parse_hex_arg(args.pop_front().expect("this shouldnt be possible"));
        mem.memory[target as usize] = char;
        target += 1;
    }
}

pub fn prns(instr: Vec<&str>) {
    let mem = lock!();

    let mut target = parse_hex_arg(instr[1]);

    while mem.memory[target as usize] != 0 {
        print!("{}", mem.memory[target as usize] as u8 as char);
        target += 1;
    }
}

pub fn inc(instr: Vec<&str>) {
    if instr.len() != 2 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let mut num = mem.clone().get_reg_str(instr[1]);
    num += 1;

    mem.update_reg_str(instr[1], num);
}

pub fn dec(instr: Vec<&str>) {
    if instr.len() != 2 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();

    let mut num = mem.clone().get_reg_str(instr[1]);
    num -= 1;

    mem.update_reg_str(instr[1], num);
}

pub fn add(instr: Vec<&str>) {
    if instr.len() != 4 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();
    let input1 = mem.clone().get_reg_str(instr[1]);
    let input2 = mem.clone().get_reg_str(instr[2]);

    mem.update_reg_str(instr[3], input1 + input2);
}

pub fn sub(instr: Vec<&str>) {
    if instr.len() != 4 {
        report_error("wrong number of arguments given");
    }

    let mut mem = lock!();
    let input1 = mem.clone().get_reg_str(instr[1]);
    let input2 = mem.clone().get_reg_str(instr[2]);

    mem.update_reg_str(instr[3], input1 - input2);
}

pub fn jz(instr: Vec<&str>) -> bool {
    if instr.len() != 3 {
        report_error("wrong number of arguments given");
    }
    let mut mem = lock!();

    let val = mem.clone().get_reg_str(instr[1]);
    if val == 0 {
        let pc = mem.reg_pc;
        mem.stack.push(pc);
        mem.reg_pc = parse_hex_arg(instr[2]);
        return false;
    }
    return true;
}
