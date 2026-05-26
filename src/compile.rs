use std::io::BufRead;

#[allow(clippy::enum_glob_use)]
use crate::Inst::{self, *};
use crate::{Error, error::Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompileConfig {
    pub optimization: u8,
    pub enable_debug: bool,
}

pub fn compile<R: BufRead>(input: R, config: &CompileConfig) -> Result<Vec<Inst>> {
    let mut code = Vec::new();
    let mut loop_body_addrs = Vec::new();

    for char_result in input.bytes() {
        let char = char_result?;
        let pc = u16::try_from(code.len()).map_err(|_| Error::MaxCodeSizeExceeded)?;
        let last_inst = code.last_mut();

        match char {
            b'>' => {
                if config.optimization >= 1
                    && let Some(Fwd(r)) = last_inst
                    && *r != u8::MAX
                {
                    *r += 1;
                } else {
                    code.push(Fwd(1));
                }
            }
            b'<' => {
                if config.optimization >= 1
                    && let Some(Rev(l)) = last_inst
                    && *l != u8::MAX
                {
                    *l += 1;
                } else {
                    code.push(Rev(1));
                }
            }
            b'+' => {
                if config.optimization >= 1
                    && let Some(Add(n)) = last_inst
                    && *n != u8::MAX
                {
                    *n += 1;
                } else {
                    code.push(Add(1));
                }
            }
            b'-' => {
                if config.optimization >= 1
                    && let Some(Sub(n)) = last_inst
                    && *n != u8::MAX
                {
                    *n += 1;
                } else {
                    code.push(Sub(1));
                }
            }
            b',' => code.push(Get),
            b'.' => code.push(Put),
            b'[' => {
                let loop_body_addr = pc.checked_add(1).ok_or(Error::MaxCodeSizeExceeded)?;
                loop_body_addrs.push(loop_body_addr);
                code.push(Jz(0));
            }
            b']' => {
                if let Some(loop_body_addr) = loop_body_addrs.pop() {
                    let loop_after_addr = pc.checked_add(1).ok_or(Error::MaxCodeSizeExceeded)?;
                    let loop_start_addr = loop_body_addr - 1;
                    let loop_start_inst = code.get_mut(loop_start_addr as usize);
                    if let Some(Jz(addr)) = loop_start_inst {
                        *addr = loop_after_addr;
                    } else {
                        panic!("Expected `jz <n>`, got {loop_start_inst:?}");
                    }

                    let loop_body = &code[(loop_body_addr as usize)..];
                    if config.optimization >= 2
                        && let Some(new_code) = optimize_loop(loop_body)
                    {
                        code.truncate(loop_start_addr as usize);
                        code.extend(new_code);
                    } else {
                        code.push(Jnz(loop_body_addr));
                    }
                } else {
                    return Err(Error::UnmatchedLoopEnd);
                }
            }
            b'#' if config.enable_debug => {
                code.push(Dbg);
            }
            _ => {}
        }
    }

    if !loop_body_addrs.is_empty() {
        return Err(Error::UnmatchedLoopStart);
    }

    Ok(code)
}

fn optimize_loop(code: &[Inst]) -> Option<Vec<Inst>> {
    match *code {
        // []
        [] => Some(vec![]),

        // [-] | [+]
        [Sub(_) | Add(_)] => Some(vec![(Clr)]),

        // [>]
        [Fwd(r)] => Some(vec![(Fzr(r))]),

        // [<]
        [Rev(l)] => Some(vec![(Fzl(l))]),

        // [->+<] | [>+<-]
        [Sub(1), Fwd(r), Add(n), Rev(l)] | [Fwd(r), Add(n), Rev(l), Sub(1)] if l == r => {
            Some(vec![(Mvr(r, n))])
        }

        // [-<+>] | [<+>-]
        [Sub(1), Rev(l), Add(n), Fwd(r)] | [Rev(l), Add(n), Fwd(r), Sub(1)] if l == r => {
            Some(vec![(Mvl(l, n))])
        }

        // _
        _ => None,
    }
}
