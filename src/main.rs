use std::{
    env,
    fs::File, io::BufReader,
};

use brainfork::{CompileConfig, EofBehavior, RunConfig, compile, run};

fn run_file(path: &str, compile_config: CompileConfig, run_config: RunConfig) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let code = compile(reader, compile_config)?;
    run(&code, run_config)
}

fn main() -> Result<(), String> {
    let compile_config = CompileConfig {
        enable_debug: true,
    };

    let run_config = RunConfig {
        eof_behavior: EofBehavior::Zero,
        debug_length: 10,
    };

    let args = env::args().skip(1).collect::<Vec<_>>();
    match args[..] {
        [ref path] => run_file(path, compile_config, run_config),
        _ => Err(format!("Expected 1 arg, got {}", args.len())),
    }
}
