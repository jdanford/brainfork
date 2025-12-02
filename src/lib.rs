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

pub use compile::{compile, CompileConfig};
pub use inst::Inst;
pub use run::{run, EofBehavior, RunConfig};
