// #![deny(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::new_without_default,
    clippy::similar_names
)]

mod compile;
mod inst;
mod run;

pub use compile::{CompileConfig, compile};
pub use inst::{Inst, dump_asm, dump_ebf};
pub use run::{EofBehavior, RunConfig, run};
