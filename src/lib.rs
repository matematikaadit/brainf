use std::io::{self, Read, Write};

/// Trait for our VM Input/Output
pub trait IO {
    /// Get one byte from input
    fn get(&mut self) -> u8;
    /// Put one byte into output
    fn put(&mut self, byte: u8);
}

/// Input/Output using io::stdin() and io::stdout()
pub struct StdIO;

impl IO for StdIO {
    fn get(&mut self) -> u8 {
        let mut buff = [0; 1];
        match io::stdin().read(&mut buff) {
            Ok(0) => 0,
            Ok(_) => buff[0],
            Err(e) => {
                // print error, but return 0 (null)
                eprintln!("IO Error in `get`: {}", e);
                0
            }
        }
    }
    fn put(&mut self, byte: u8) {
        match io::stdout().write(&[byte]) {
            Ok(_) => (),
            Err(e) => eprintln!("IO Error in `put`: {}", e),
        }
    }
}

/// Maximum length of VM's memory size
const MAX_LEN: usize = 30_000; // recommended size

/// Brainfuck VM
pub struct Vm<T> {
    /// Pointer to be manipulated by the instructions
    p: usize,
    /// Memory for holding the data
    mem: Vec<u8>,
    /// Return stack for holding the jump location in the `[` and `]` instruction pair
    rstack: Vec<usize>,
    /// Input/Output for the `.` and `,` instruction
    io: T,
}

impl<T> Vm<T> where T: IO {
    /// Create a new brainfuck VM
    pub fn new(io: T) -> Vm<T> {
        Vm {
            p: 0,
            mem: vec![0; MAX_LEN],
            rstack: Vec::new(),
            io: io,
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
                b'.' => self.io.put(self.mem[self.p]),
                b',' => self.mem[self.p] = self.io.get(),
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

/// Create default VM using IO from StdIO
pub fn default_vm() -> Vm<StdIO> {
    Vm::new(StdIO)
}

#[cfg(test)]
mod test {
    use crate::IO;
    use crate::Vm;

    struct TestIO(Vec<u8>);

    impl IO for &mut TestIO {
        fn get(&mut self) -> u8 {
            0
        }
        fn put(&mut self, byte: u8) {
            self.0.push(byte)
        }
    }

    #[test]
    fn test_hello_world() {
        // taken from wikipedia
        let hello_world = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mut test_io = TestIO(Vec::new());
        {
            // on scope so we can get back the mutably borrowed test_io when it finished
            let mut vm = Vm::new(&mut test_io);
            assert_eq!(vm.run(hello_world), Ok(()));
        }
        assert_eq!(test_io.0, b"Hello World!\n");
    }
}
