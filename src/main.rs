use std::io::{self, Read, Write};
use std::env;
use std::process;
use std::fs::File;

const MAX_LEN: usize = 30_000;

struct Vm {
    p: usize,
    mem: Vec<u8>,
    rstack: Vec<usize>,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            p: 0,
            mem: vec![0; MAX_LEN],
            rstack: Vec::new(),
        }
    }
    fn right(&mut self) {
        self.p = (self.p + 1) % MAX_LEN;
    }
    fn left(&mut self) {
        self.p = (self.p + MAX_LEN - 1) % MAX_LEN; // prevent underflow provided usize max >= 2 * 30_000
    }
    fn inc(&mut self) {
        self.mem[self.p] = self.mem[self.p].wrapping_add(1);
    }
    fn dec(&mut self) {
        self.mem[self.p] = self.mem[self.p].wrapping_sub(1);
    }
    fn put(&self) {
        let buff = [self.mem[self.p]];
        match io::stdout().write(&buff) {
            Ok(_) => (),
            Err(e) => eprintln!("IO Error in `put`: {}", e),
        }
    }
    fn get(&mut self) {
        let mut buff = [0; 1];
        match io::stdin().read(&mut buff) {
            Ok(0) => self.mem[self.p] = 0,
            Ok(_) => self.mem[self.p] = buff[0],
            Err(e) => eprintln!("IO Error in `get`: {}", e),
        }
    }
    fn is_zero(&self) -> bool {
        self.mem[self.p] == 0
    }
    fn run(&mut self, script: &[u8]) -> Result<(), usize> {
        let mut cursor = 0;
        while cursor < script.len() {
            let byte = script[cursor];
            let mut branching = false;
            match byte {
                b'<' => self.left(),
                b'>' => self.right(),
                b'+' => self.inc(),
                b'-' => self.dec(),
                b'.' => self.put(),
                b',' => self.get(),
                b'[' => self.rstack.push(cursor+1),
                b']' => branching = true,
                _ => (),
            }
            if branching {
                let jump = !self.is_zero();
                match (self.rstack.last(), jump) {
                    // loop unfinished, jump back
                    (Some(&n), true) => {
                        cursor = n;
                    }
                    // loop finished, continue to next byte
                    (Some(_), false) => {
                        cursor += 1;
                        self.rstack.pop(); // don't forget to pop that jump address
                    }
                    (None, _) => return Err(cursor),
                }
            } else {
                cursor += 1;
            }
        }
        if self.rstack.is_empty() {
            Ok(())
        } else {
            Err(self.rstack[self.rstack.len()-1])
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: brainf <file>");
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

    let mut vm = Vm::new();
    match vm.run(&script) {
        Err(pos) => {
            eprintln!("Error, unmatched bracket at {}", pos);
            process::exit(1);
        }
        Ok(()) => (),
    }
}
