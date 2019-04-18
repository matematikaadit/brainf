use std::io::{self, Read, Write};

/// Maximum length of VM's memory size
const MAX_LEN: usize = 30_000; // recommended size

/// Brainfuck VM
pub struct Vm {
    /// Pointer to be manipulated by the instructions
    p: usize,
    /// Memory for holding the data
    mem: Vec<u8>,
    /// Return stack for holding the jump location in the `[` and `]` instruction pair
    rstack: Vec<usize>,
}

impl Vm {
    /// Create a new brainfuck VM
    pub fn new() -> Vm {
        Vm {
            p: 0,
            mem: vec![0; MAX_LEN],
            rstack: Vec::new(),
        }
    }

    /// Move the pointer to the right, wraps around in case of overflow
    fn right(&mut self) {
        self.p = (self.p + 1) % MAX_LEN;
    }

    /// Move the pointer to the left, wraps around in case of overflow
    fn left(&mut self) {
        // prevent underflow, provided usize max >= 2 * 30_000
        self.p = (self.p + MAX_LEN - 1) % MAX_LEN;
    }

    /// Add one to the currently pointed value
    fn inc(&mut self) {
        self.mem[self.p] = self.mem[self.p].wrapping_add(1);
    }

    /// Substract one from the currently pointed value
    fn dec(&mut self) {
        self.mem[self.p] = self.mem[self.p].wrapping_sub(1);
    }

    /// Output currently pointed value into stdout
    fn put(&self) {
        let buff = [self.mem[self.p]];
        match io::stdout().write(&buff) {
            Ok(_) => (),
            Err(e) => eprintln!("IO Error in `put`: {}", e),
        }
    }

    /// Get one byte from the input and put it in the currently pointed value
    fn get(&mut self) {
        let mut buff = [0; 1];
        match io::stdin().read(&mut buff) {
            Ok(0) => self.mem[self.p] = 0,
            Ok(_) => self.mem[self.p] = buff[0],
            Err(e) => eprintln!("IO Error in `get`: {}", e),
        }
    }

    /// Check if currently pointed value is zero
    fn is_zero(&self) -> bool {
        self.mem[self.p] == 0
    }

    /// Run a brainfuck script, in case of unmatched bracket, return the unmatched location
    pub fn run(&mut self, script: &[u8]) -> Result<(), usize> {
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
                        self.rstack.pop(); // clear this jump
                        cursor += 1;
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
