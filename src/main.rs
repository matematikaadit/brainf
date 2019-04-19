extern crate brainf;

use std::io::Read;
use std::env;
use std::process;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(r#"brainf v0.1: brainfuck interpreter.

USAGE: brainf <file>

* <file> : brainfuck program to run."#);
        process::exit(1);
    }

    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1)
        }
    };

    let mut script = Vec::new();
    match file.read_to_end(&mut script) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }

    let mut vm = brainf::default_vm();

    match vm.run(&script) {
        Err(pos) => {
            eprintln!("Error, unmatched bracket at {}", pos);
            process::exit(1);
        }
        Ok(()) => (),
    }
}
