use std::{
    env,
    fs::File, io::BufReader,
};

use brainfork::{EofBehavior, RunConfig, compile, run};

fn run_file(path: &str, config: RunConfig) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let code = compile(reader)?;

    // println!("--------------------------------");
    // dump(&code);
    // println!("--------------------------------");

    run(&code, config)
}

fn main() -> Result<(), String> {
    let config = RunConfig {
        eof_behavior: EofBehavior::Zero,
    };

    let args = env::args().skip(1).collect::<Vec<_>>();
    match args[..] {
        [ref path] => run_file(path, config),
        _ => Err(format!("Expected 1 arg, got {}", args.len())),
    }
}
