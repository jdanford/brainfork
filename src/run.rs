use std::io::{self, Read, Write};

use crate::Inst;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EofBehavior {
    Zero,
    #[allow(dead_code)]
    Neg1,
    #[allow(dead_code)]
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunConfig {
    pub memory_size: u16,
    pub eof_behavior: EofBehavior,
    pub debug_length: usize,
}

pub fn run(code: &[Inst], config: RunConfig) -> Result<(), String> {
    let mut memory = vec![0u8; config.memory_size as usize];
    let mut buffer = [0u8; 1];

    let mut ip: u16 = 0;
    let mut dp: u16 = 0;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    while let Some(&inst) = code.get(ip as usize) {
        match inst {
            Inst::Fwd(n) => {
                dp = dp.wrapping_add(u16::from(n));
            }
            Inst::Rev(n) => {
                dp = dp.wrapping_sub(u16::from(n));
            }
            Inst::Add(n) => {
                let cell = memory.get_mut(dp as usize).unwrap();
                *cell = cell.wrapping_add(n);
            }
            Inst::Sub(n) => {
                let cell = memory.get_mut(dp as usize).unwrap();
                *cell = cell.wrapping_sub(n);
            }
            Inst::Clr => {
                let cell = memory.get_mut(dp as usize).unwrap();
                *cell = 0;
            }
            Inst::Mvr(r, n) => {
                let i = dp as usize;
                let j = (dp + u16::from(r)) as usize;
                memory[j] = memory[j].wrapping_add(memory[i].wrapping_mul(n));
                memory[i] = 0;
            }
            Inst::Mvl(l, n) => {
                let i = dp as usize;
                let j = (dp - u16::from(l)) as usize;
                memory[j] = memory[j].wrapping_add(memory[i].wrapping_mul(n));
                memory[i] = 0;
            }
            Inst::Fzr(n) => {
                while memory[dp as usize] != 0 {
                    dp += u16::from(n);
                }
            }
            Inst::Fzl(n) => {
                while memory[dp as usize] != 0 {
                    dp -= u16::from(n);
                }
            }
            Inst::Get => {
                let cell = memory.get_mut(dp as usize).unwrap();
                match stdin.read_exact(&mut buffer) {
                    Ok(()) => *cell = buffer[0],
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => match config.eof_behavior
                    {
                        EofBehavior::Zero => *cell = 0,
                        EofBehavior::Neg1 => *cell = u8::MAX,
                        EofBehavior::Unchanged => {}
                    },
                    Err(e) => return Err(e.to_string()),
                }
            }
            Inst::Put => {
                let cell = memory.get_mut(dp as usize).unwrap();
                buffer[0] = *cell;
                stdout.write_all(&buffer).unwrap();
            }
            Inst::Jz(new_ip) => {
                let cell = memory.get_mut(dp as usize).unwrap();
                if *cell == 0 {
                    ip = new_ip;
                    continue;
                }
            }
            Inst::Jnz(new_ip) => {
                let cell = memory.get_mut(dp as usize).unwrap();
                if *cell != 0 {
                    ip = new_ip;
                    continue;
                }
            }
            Inst::Dbg => {
                let view = &memory[..config.debug_length];
                for (i, byte) in view.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }

                    print!("{byte:02x}");
                }

                let offset = dp as usize * 3 + 2;
                println!("\n{:width$}", "^", width = offset);
            }
            Inst::Nop => {}
        }

        ip += 1;
    }

    Ok(())
}
