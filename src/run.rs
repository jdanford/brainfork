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
        let cell = memory.get_mut(dp as usize).unwrap();

        match inst {
            Inst::Fwd(n) => {
                dp = dp.wrapping_add(n);
            }
            Inst::Rev(n) => {
                dp = dp.wrapping_sub(n);
            }
            Inst::Add(n) => {
                *cell = cell.wrapping_add(n);
            }
            Inst::Sub(n) => {
                *cell = cell.wrapping_sub(n);
            }
            Inst::Get => match stdin.read_exact(&mut buffer) {
                Ok(()) => *cell = buffer[0],
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => match config.eof_behavior {
                    EofBehavior::Zero => *cell = 0,
                    EofBehavior::Neg1 => *cell = u8::MAX,
                    EofBehavior::Unchanged => {}
                },
                Err(e) => return Err(e.to_string()),
            },
            Inst::Put => {
                buffer[0] = *cell;
                stdout.write_all(&buffer).unwrap();
            }
            Inst::Jz(new_ip) => {
                if *cell == 0 {
                    ip = new_ip;
                    continue;
                }
            }
            Inst::Jnz(new_ip) => {
                if *cell != 0 {
                    ip = new_ip;
                    continue;
                }
            }
            Inst::Dbg => {
                let view = &memory[..config.debug_length];
                for byte in view {
                    print!("{byte:02x} ");
                }

                let offset = dp as usize * 3 + 2;
                println!("\n{:width$}", "^", width = offset);
            }
        }

        ip += 1;
    }

    Ok(())
}
