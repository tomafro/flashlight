#![feature(non_modrs_mods)]
#![feature(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate regex;

mod buffer;
mod parser;

pub use parser::Line;
pub use buffer::Buffer;
pub use parser::Context;