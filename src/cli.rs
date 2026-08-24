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

    #[arg(short = 'O', long, value_parser = value_parser!(u8).range(0..=2), default_value_t = 2)]
    opt: u8,

    #[arg(short = 'd', long)]
    debug: bool,

    #[arg(short = 'e', long, default_value_t = EofBehavior::Unchanged)]
    eof: EofBehavior,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let compile_config = CompileConfig {
        optimization: args.opt,
        enable_debug: args.debug,
    };

    let mut run_config = RunConfig {
        stdin: io::stdin(),
        stdout: io::stdout(),
        eof_behavior: args.eof,
        debug_length: 10,
    };

    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let code = compile(reader, &compile_config)?;
    run(&code, &mut run_config)?;
    Ok(())
}
