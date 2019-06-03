// new implementation

use std::io;
use std::io::Write;
use std::io::Read;
use std::num::Wrapping;


/// Maximum length for the brainfuck program's memory
const MAXLEN: usize = 30_000; // recommended size


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

    let mut data = vec![Wrapping(0u8); MAXLEN];
    let mut dptr = 0;

    loop {
        match src.get(iptr) {
            Some(b'+') => {
                data[dptr] += Wrapping(1);
                iptr += 1;
            }
            Some(b'-') => {
                data[dptr] -= Wrapping(1);
                iptr += 1;
            }
            Some(b'<') => {
                // avoiding underflow
                dptr = dptr.checked_sub(1).unwrap_or_else(|| MAXLEN);
                iptr += 1;
            }
            Some(b'>') => {
                dptr = (dptr + 1) % MAXLEN;
                iptr += 1;
            }
            Some(b',') => {
                let mut buff = [0];
                input.read(&mut buff).map_err(|e| Error::Input(e))?;
                data[dptr] = Wrapping(buff[0]);
                iptr += 1;
            }
            Some(b'.') => {
                output.write(&[data[dptr].0]).map_err(|e| Error::Output(e))?;
                iptr += 1;
            }
            Some(b'[') => {
                rstack.push(iptr);
                iptr += 1;
            }
            Some(b']') => {
                match rstack.last() {
                    Some(&ret) => {
                        if data[dptr].0 == 0 {
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
