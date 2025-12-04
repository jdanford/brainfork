use std::{env, fs::File, io::BufReader, path::Path};

use brainfork::{CompileConfig, EofBehavior, RunConfig, compile, run};

fn run_file(
    path: &Path,
    compile_config: CompileConfig,
    run_config: RunConfig,
) -> Result<(), String> {
    let file = File::open(path).map_err(|err| err.to_string())?;
    let reader = BufReader::new(file);
    let code = compile(reader, compile_config)?;
    run(&code, run_config)?;
    Ok(())
}

fn main() -> Result<(), String> {
    let compile_config = CompileConfig {
        optimization: 2,
        enable_debug: true,
    };

    let run_config = RunConfig {
        memory_size: u16::MAX,
        eof_behavior: EofBehavior::Zero,
        debug_length: 10,
    };

    let args = env::args().skip(1).collect::<Vec<_>>();
    match args[..] {
        [ref path_str] => run_file(Path::new(path_str), compile_config, run_config),
        _ => Err(format!("Expected 1 arg, got {}", args.len())),
    }
}
