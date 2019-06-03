use std::io::Read;
use std::env;
use std::process::exit;
use std::fs::File;

use brainf::Error;

fn main() {
    let mut args = env::args().skip(1); // skip args[0] (program name)
    let script_name = match args.next() {
        Some(s) => s,
        None => {
            usage();
            exit(1);
        }
    };

    let mut file = match File::open(script_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening script: {}", e);
            exit(1)
        }
    };

    let mut script = Vec::new();
    match file.read_to_end(&mut script) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading script: {}", e);
            exit(1);
        }
    }

    match brainf::eval_with_stdio(&script) {
        Err(Error::RBrac(pos)) => {
            eprintln!("Error, unmatched ] at position: {}", pos);
            exit(1);
        }
        Err(Error::LBrac(positions)) => {
            eprint!("Error, unmatched [ at position: ");
            for pos in positions {
                eprint!("{}, ", pos);
            }
            eprintln!();
            exit(1);
        }
        Err(Error::Input(e)) => {
            eprintln!("Error reading from stdin: {}", e);
            exit(1);
        }
        Err(Error::Output(e)) => {
            eprintln!("Error writing to stdout: {}", e); // and hoping this eprintln also not error
            exit(1);
        }
        Ok(()) => (),
    }
}

fn usage() {
    eprintln!(
r#"brainf v0.2: brainfuck interpreter.

USAGE: brainf <file>

* <file> : brainfuck program to run.
"#);
}
