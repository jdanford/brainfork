use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inst {
    Fwd(u8),
    Rev(u8),
    Add(u8),
    Sub(u8),
    Clr,
    Mvr(u8, u8),
    Mvl(u8, u8),
    Fzr(u8),
    Fzl(u8),
    Get,
    Put,
    Jz(u16),
    Jnz(u16),
    Dbg,
    Nop,
}

impl Inst {
    fn fmt_ebf<W: Write>(self, f: &mut W) -> io::Result<()> {
        match self {
            Inst::Fwd(n) => write!(f, ">{n}"),
            Inst::Rev(n) => write!(f, "<{n}"),
            Inst::Add(n) => write!(f, "+{n}"),
            Inst::Sub(n) => write!(f, "-{n}"),
            Inst::Clr => write!(f, "[-]"),
            Inst::Mvr(r, n) => write!(f, "+>{r}x{n}"),
            Inst::Mvl(l, n) => write!(f, "<+{l}x{n}"),
            Inst::Fzr(n) => write!(f, "[>]{n}"),
            Inst::Fzl(n) => write!(f, "[<]{n}"),
            Inst::Get => write!(f, ","),
            Inst::Put => write!(f, "."),
            Inst::Jz(_) => write!(f, "["),
            Inst::Jnz(_) => write!(f, "]"),
            Inst::Dbg => write!(f, "#"),
            Inst::Nop => write!(f, "_"),
        }
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Inst::Fwd(n) => write!(f, "fwd {n}"),
            Inst::Rev(n) => write!(f, "rev {n}"),
            Inst::Add(n) => write!(f, "add {n}"),
            Inst::Sub(n) => write!(f, "sub {n}"),
            Inst::Clr => write!(f, "clr"),
            Inst::Mvr(r, n) => write!(f, "mvr {r} {n}"),
            Inst::Mvl(l, n) => write!(f, "mvl {l} {n}"),
            Inst::Fzr(n) => write!(f, "fzr {n}"),
            Inst::Fzl(n) => write!(f, "fzl {n}"),
            Inst::Get => write!(f, "get"),
            Inst::Put => write!(f, "put"),
            Inst::Jz(addr) => write!(f, "jz  {addr}"),
            Inst::Jnz(addr) => write!(f, "jnz {addr}"),
            Inst::Dbg => write!(f, "dbg"),
            Inst::Nop => write!(f, "nop"),
        }
    }
}

#[allow(dead_code)]
pub fn dump_asm<W: Write>(code: &[Inst], writer: &mut W) -> Result<(), io::Error> {
    for (pc, inst) in code.iter().enumerate() {
        writeln!(writer, "{pc:>5}: {inst}")?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn dump_ebf<W: Write>(code: &[Inst], writer: &mut W) -> Result<(), io::Error> {
    let mut depth = 0;
    let mut line = 0;
    let mut col = 0;

    for inst in code {
        match inst {
            Inst::Jz(_) => {
                col = 0;

                let indent = depth * 2;
                write!(writer, "\n{:width$}", "", width = indent)?;
                write!(writer, "[")?;

                depth += 1;
                line += 1;
            }
            Inst::Jnz(_) => {
                col = 0;
                line += 1;
                depth -= 1;

                let indent = depth * 2;
                write!(writer, "\n{:width$}", "", width = indent)?;
                write!(writer, "]")?;
            }
            _ => {
                if line > 0 && col == 0 {
                    writeln!(writer)?;
                    line += 1;
                }

                let indent = depth * 2;
                let indent_or_space = if col == 0 { indent } else { 1 };
                write!(writer, "{:width$}", "", width = indent_or_space)?;
                inst.fmt_ebf(writer)?;

                col += 1;
            }
        }
    }

    writeln!(writer)?;
    Ok(())
}
