mod instructions;
mod memory;
mod parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./gooberator program-file-path ref-file-path");
        return;
    }

    parse::parse_start(&args[1], &args[2]);
}
