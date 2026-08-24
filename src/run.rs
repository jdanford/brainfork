use std::{
    fmt::{self, Debug, Display},
    io::{self, Read, Write},
    str::FromStr,
};

use crate::{Inst, Result};

const MEMORY_SIZE: usize = 1 << u16::BITS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EofBehavior {
    Zero,
    Neg1,
    Unchanged,
}

impl Display for EofBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EofBehavior::Zero => write!(f, "zero"),
            EofBehavior::Neg1 => write!(f, "neg1"),
            EofBehavior::Unchanged => write!(f, "unchanged"),
        }
    }
}

impl FromStr for EofBehavior {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "zero" => Ok(EofBehavior::Zero),
            "neg1" => Ok(EofBehavior::Neg1),
            "unchanged" => Ok(EofBehavior::Unchanged),
            _ => Err("invalid value".to_owned()),
        }
    }
}

pub struct RunConfig<I: Read, O: Write> {
    pub stdin: I,
    pub stdout: O,
    pub eof_behavior: EofBehavior,
    pub debug_length: usize,
}

#[allow(clippy::too_many_lines)]
pub fn run<I: Read, O: Write>(code: &[Inst], config: &mut RunConfig<I, O>) -> Result<()> {
    let mut mem = vec![0u8; MEMORY_SIZE];
    let mut io_buf = [0u8; 1];

    let mut pc: u16 = 0;
    let mut dp: u16 = 0;

    while let Some(&inst) = code.get(pc as usize) {
        match inst {
            Inst::Fwd(n) => {
                dp = dp.wrapping_add(u16::from(n));
            }
            Inst::Rev(n) => {
                dp = dp.wrapping_sub(u16::from(n));
            }
            Inst::Add(n) => {
                let dest = dp as usize;
                mem[dest] = mem[dest].wrapping_add(n);
            }
            Inst::Sub(n) => {
                let dest = dp as usize;
                mem[dest] = mem[dest].wrapping_sub(n);
            }
            Inst::Clr => {
                let dest = dp as usize;
                mem[dest] = 0;
            }
            Inst::Mvr(r, n) => {
                let src = dp as usize;
                let dest = (dp.wrapping_add(u16::from(r))) as usize;
                let value = mem[src].wrapping_mul(n);
                mem[dest] = mem[dest].wrapping_add(value);
                mem[src] = 0;
            }
            Inst::Mvl(l, n) => {
                let src = dp as usize;
                let dest = (dp.wrapping_sub(u16::from(l))) as usize;
                let value = mem[src].wrapping_mul(n);
                mem[dest] = mem[dest].wrapping_add(value);
                mem[src] = 0;
            }
            Inst::Fzr(n) => {
                while mem[dp as usize] != 0 {
                    dp = dp.wrapping_add(u16::from(n));
                }
            }
            Inst::Fzl(n) => {
                while mem[dp as usize] != 0 {
                    dp = dp.wrapping_sub(u16::from(n));
                }
            }
            Inst::Get => {
                let dest = dp as usize;
                match config.stdin.read_exact(&mut io_buf) {
                    Ok(()) => mem[dest] = io_buf[0],
                    Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                        match config.eof_behavior {
                            EofBehavior::Zero => mem[dest] = 0,
                            EofBehavior::Neg1 => mem[dest] = u8::MAX,
                            EofBehavior::Unchanged => {}
                        }
                    }
                    Err(err) => return Err(err.into()),
                }
            }
            Inst::Put => {
                let dest = dp as usize;
                io_buf[0] = mem[dest];
                config.stdout.write_all(&io_buf)?;
            }
            Inst::Jz(new_pc) => {
                let dest = dp as usize;
                if mem[dest] == 0 {
                    pc = new_pc;
                    continue;
                }
            }
            Inst::Jnz(new_pc) => {
                let dest = dp as usize;
                if mem[dest] != 0 {
                    pc = new_pc;
                    continue;
                }
            }
            Inst::Dbg => {
                let view = &mem[..config.debug_length];
                for (i, byte) in view.iter().enumerate() {
                    if i == 0 {
                        writeln!(config.stdout,)?;
                    } else {
                        write!(config.stdout, " ")?;
                    }

                    write!(config.stdout, "{byte:02x}")?;
                }

                if (dp as usize) < config.debug_length {
                    let offset = dp as usize * 3 + 1;
                    writeln!(config.stdout, "\n{:>width$}", "^", width = offset)?;
                } else {
                    writeln!(config.stdout)?;
                }
            }
            Inst::Nop => {}
        }

        pc += 1;
    }

    Ok(())
}
