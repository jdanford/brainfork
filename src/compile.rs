use std::io::BufRead;

use crate::Inst;

const ERROR_MAX_CODE_SIZE: &str = "Maximum code size exceeded";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompileConfig {
    pub enable_debug: bool,
}

pub fn compile<R: BufRead>(input: R, config: CompileConfig) -> Result<Vec<Inst>, String> {
    let mut code = Vec::new();
    let mut loop_starts = Vec::new();

    for char_result in input.bytes() {
        let char = char_result.map_err(|err| err.to_string())?;
        let pc = u16::try_from(code.len()).map_err(|_| ERROR_MAX_CODE_SIZE)?;
        let last_inst = code.last_mut();

        match char {
            b'>' => {
                if let Some(Inst::Fwd(n)) = last_inst
                    && *n != u16::MAX
                {
                    *n += 1;
                } else {
                    code.push(Inst::Fwd(1));
                }
            }
            b'<' => {
                if let Some(Inst::Rev(n)) = last_inst
                    && *n != u16::MAX
                {
                    *n += 1;
                } else {
                    code.push(Inst::Rev(1));
                }
            }
            b'+' => {
                if let Some(Inst::Add(n)) = last_inst
                    && *n != u8::MAX
                {
                    *n += 1;
                } else {
                    code.push(Inst::Add(1));
                }
            }
            b'-' => {
                if let Some(Inst::Sub(n)) = last_inst
                    && *n != u8::MAX
                {
                    *n += 1;
                } else {
                    code.push(Inst::Sub(1));
                }
            }
            b',' => code.push(Inst::Get),
            b'.' => code.push(Inst::Put),
            b'[' => {
                let loop_start = pc.checked_add(1).ok_or(ERROR_MAX_CODE_SIZE)?;
                loop_starts.push(loop_start);
                code.push(Inst::Jz(0));
            }
            b']' => {
                if let Some(loop_start) = loop_starts.pop() {
                    let loop_end = pc.checked_add(1).ok_or(ERROR_MAX_CODE_SIZE)?;
                    let loop_start_pc = loop_start - 1;
                    let loop_start_inst = code.get_mut(loop_start_pc as usize);
                    if let Some(Inst::Jz(addr)) = loop_start_inst {
                        *addr = loop_end;
                    } else {
                        unreachable!("Expected `jz`, got {loop_start_inst:?}");
                    }

                    code.push(Inst::Jnz(loop_start));
                } else {
                    return Err("Encountered loop end without matching loop start".to_string());
                }
            }
            b'#' if config.enable_debug => {
                code.push(Inst::Dbg);
            }
            _ => {}
        }
    }

    if !loop_starts.is_empty() {
        return Err("Encountered loop start without matching loop end".to_string());
    }

    Ok(code)
}
