use std::fmt;
use std::fmt::Display;

pub enum Instr {
    Add(u8),
    Sub(u8),
    Next(usize),
    Prev(usize),
    Get,
    Put,
    Open,
    Close,
}

pub impl Display for Instr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instr::Add(n) => {
                for _ in 0..n {
                    write!(fmt, "+")?;
                }
            }
            Instr::Sub(n) => {
                for _ in 0..n {
                    write!(fmt, "-")?;
                }
            }
            Instr::Next(n) => {
                for _ in 0..n {
                    write!(fmt, ">")?;
                }
            }
            Instr::Prev(n) => {
                for _ in 0..n {
                    write!(fmt, "<")?;
                }
            }
            Instr::Get => {
                write!(fmt, ",")?;
            }
            Instr::Put => {
                write!(fmt, ".")?;
            }
            Instr::Open => {
                write!(fmt, "[")?;
            }
            Instr::Close => {
                write!(fmt, "]")?;
            }
        }
        Ok(())
    }
}

pub fn print(program: &[Instr]) {
    for inst in program {
        print!("{}", inst);
    }
}
