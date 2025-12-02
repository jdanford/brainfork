use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inst {
    Fwd(u16),
    Rev(u16),
    Add(u8),
    Sub(u8),
    Get,
    Put,
    Jz(u16),
    Jnz(u16),
}

impl Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Inst::Fwd(n) => write!(f, "fwd {n}"),
            Inst::Rev(n) => write!(f, "rev {n}"),
            Inst::Add(n) => write!(f, "add {n}"),
            Inst::Sub(n) => write!(f, "sub {n}"),
            Inst::Get => write!(f, "get"),
            Inst::Put => write!(f, "put"),
            Inst::Jz(pc) => write!(f, "jz  {pc}"),
            Inst::Jnz(pc) => write!(f, "jnz {pc}"),
        }
    }
}

#[allow(dead_code)]
pub fn dump(code: &[Inst]) {
    for (pc, inst) in code.iter().enumerate() {
        println!("{pc:>5}: {inst}");
    }
}
