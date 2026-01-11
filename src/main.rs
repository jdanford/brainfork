use std::process::ExitCode;

use brainfork::{cli, handle_error};

fn main() -> ExitCode {
    let result = cli::main();
    handle_error(result)
}
