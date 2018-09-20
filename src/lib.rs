#![feature(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate indoc;
extern crate docopt;
extern crate isatty;
extern crate regex;

mod buffer;
mod cli;
mod matcher;
mod parser;
mod reader;
mod runner;
#[cfg(test)]
mod test;

pub use buffer::Buffer;
pub use cli::Args;
pub use cli::Config;
pub use matcher::Matcher;
pub use parser::Context;
pub use parser::Line;
pub use reader::LineReader;
pub use runner::Runner;
