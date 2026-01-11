use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use clap::{Parser, value_parser};

use crate::{CompileConfig, EofBehavior, RunConfig, compile, error::Result, run};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None,
)]
pub struct Args {
    path: PathBuf,

    #[arg(short = 'O', value_parser = value_parser!(u8).range(0..=2), default_value_t = 2)]
    optimization: u8,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let compile_config = CompileConfig {
        optimization: args.optimization,
        enable_debug: true,
    };

    let mut run_config = RunConfig {
        stdin: io::stdin(),
        stdout: io::stdout(),
        eof_behavior: EofBehavior::Unchanged,
        memory_size: 1 << 16,
        debug_length: 10,
    };

    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let code = compile(reader, &compile_config)?;
    run(&code, &mut run_config)?;
    Ok(())
}
