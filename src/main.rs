use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path,
    process::ExitCode,
};

use anyhow::anyhow;
use brainfork::{CompileConfig, EofBehavior, Result, RunConfig, compile, handle_error, run};

fn run_file<I: Read, O: Write>(
    path: &Path,
    compile_config: &CompileConfig,
    run_config: &mut RunConfig<I, O>,
) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let code = compile(reader, compile_config)?;
    run(&code, run_config)?;
    Ok(())
}

fn main() -> ExitCode {
    let compile_config = CompileConfig {
        optimization: 2,
        enable_debug: true,
    };

    let mut run_config = RunConfig {
        stdin: io::stdin(),
        stdout: io::stdout(),
        eof_behavior: EofBehavior::Unchanged,
        memory_size: 1 << 16,
        debug_length: 10,
    };

    let args = env::args().skip(1).collect::<Vec<_>>();
    let result = match &args[..] {
        [path_str] => run_file(Path::new(path_str), &compile_config, &mut run_config),
        _ => Err(anyhow!("expected 1 arg, got {}", args.len()).into()),
    };

    handle_error(result)
}
