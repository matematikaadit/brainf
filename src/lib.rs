use std::io;
use std::io::Write;
use std::io::Read;
use std::num::Wrapping;


/// Maximum length for the brainfuck program's memory
const MAXLEN: usize = 1 << 16; // for easy wrapping behaviour


/// Possible Error when evaluating the brainfuck program
pub enum Error {
    /// Unmatched right bracket (`]`)
    RBrac(usize),
    /// Unmatched left brackets ('[')
    LBrac(Vec<usize>),
    /// Input error
    Input(io::Error),
    /// Output error
    Output(io::Error),
}


/// Evaluating a brainfuck script with input/output manually provided to the function
pub fn eval<R, W>(src: &[u8], input: &mut R, output: &mut W) -> Result<(), Error>
where R: Read,
      W: Write {

    let mut iptr = 0;
    let mut rstack = Vec::new();

    let mut mem = Mem::new();

    const ONE_W8: Wrapping<u8> = Wrapping(1);
    const ONE_W16: Wrapping<u16> = Wrapping(1);

    loop {
        match src.get(iptr) {
            Some(b'+') => {
                *mem.get_mut() += ONE_W8;
                iptr += 1;
            }
            Some(b'-') => {
                *mem.get_mut() -= ONE_W8;
                iptr += 1;
            }
            Some(b'<') => {
                *mem.ptr_mut() -= ONE_W16;
                iptr += 1;
            }
            Some(b'>') => {
                *mem.ptr_mut() += ONE_W16;
                iptr += 1;
            }
            Some(b',') => {
                let mut buff = [0];
                input.read(&mut buff).map_err(|e| Error::Input(e))?;
                *mem.get_mut() = Wrapping(buff[0]);
                iptr += 1;
            }
            Some(b'.') => {
                let buff = [mem.get()];
                output.write(&buff).map_err(|e| Error::Output(e))?;
                iptr += 1;
            }
            Some(b'[') => {
                rstack.push(iptr);
                iptr += 1;
            }
            Some(b']') => {
                match rstack.last() {
                    Some(&ret) => {
                        if mem.get() == 0 {
                            // jump out
                            rstack.pop();
                            iptr += 1;
                        } else {
                            // loop
                            iptr = ret + 1;
                        }
                    },
                    None => return Err(Error::RBrac(iptr)),
                }
            }
            Some(_) => (), // ignore other character
            None => {
                break;
            }
        }
    }
    if rstack.is_empty() {
        Ok(())
    } else {
        Err(Error::LBrac(rstack))
    }
}

/// Memory for the brainfuck program
struct Mem {
    /// The place where we put the bytes
    buff: Vec<Wrapping<u8>>,
    /// Pointer to the current byte
    ptr: Wrapping<u16>,
}

impl Mem {
    fn new() -> Mem {
        Mem {
            buff: vec![Wrapping(0); MAXLEN],
            ptr: Wrapping(0),
        }
    }
    fn get_mut(&mut self) -> &mut Wrapping<u8> {
        &mut self.buff[self.ptr.0 as usize]
    }
    fn get(&self) -> u8 {
        self.buff[self.ptr.0 as usize].0
    }
    fn ptr_mut(&mut self) -> &mut Wrapping<u16> {
        &mut self.ptr
    }
}


/// Evaluating brainfuck script with the default stdin/stdout
pub fn eval_with_stdio(src: &[u8]) -> Result<(), Error> {
    eval(src, &mut io::stdin(), &mut io::stdout())
}


#[cfg(test)]
mod test {
    use super::eval;
    use std::io;
    use std::io::Cursor;

    #[test]
    fn test_hello_world() {
        // taken from wikipedia
        let hello_world = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mut output = Vec::new();
        assert!(eval(hello_world, &mut io::empty(), &mut output).is_ok());
        assert_eq!(&output, b"Hello World!\n");
    }


    #[test]
    fn adding_two_one_digit_number() {
        let script = b",>,>>++++[->++++[-<<+++<---<--->>>>]<]<<[-<+>]>[-<<+>>]<<.";
        // Note that this only works with a single digit result

        // 2 + 6 == 8
        let mut input = Cursor::new(b"26");
        let mut output = Vec::new();
        assert!(eval(script, &mut input, &mut output).is_ok());
        assert_eq!(&output, b"8");

        // 4 + 3 == 7
        let mut input = Cursor::new(b"43");
        output.clear();
        assert!(eval(script, &mut input, &mut output).is_ok());
        assert_eq!(&output, b"7");
    }
}
