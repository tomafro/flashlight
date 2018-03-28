#![feature(non_modrs_mods)]
#![feature(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate indoc;
extern crate docopt;
extern crate regex;

mod cli;
mod buffer;
mod parser;
#[cfg(test)]
mod test;

pub use cli::Args;
pub use cli::Config;
pub use parser::Line;
pub use buffer::Buffer;
pub use parser::Context;

use regex::Regex;
use std::collections::HashSet;
use std::io::{BufRead, Write};

pub fn run<T: BufRead, U: Write>(config: Config, input: T, output: &mut U) {
    let mut buffer = Buffer::new(config.buffer_size);
    let mut matched_requests = HashSet::new();

    for input_line in input.lines() {
        let line = Line::from(input_line.unwrap());

        if config.contexts.is_empty() || config.contexts.contains(line.context()) {
            if matched_requests.contains(line.request_id()) {
                write!(output, "{}\n", line.content()).unwrap();
            } else if config.matcher.is_match(line.content()) {
                for previous in buffer
                    .lines()
                    .into_iter()
                    .filter(|l| l.request_id() == line.request_id())
                {
                    write!(output, "{}\n", previous.content()).unwrap();
                }
                write!(output, "{}\n", line.content()).unwrap();
                matched_requests.insert(line.request_id().clone());
            }
            buffer.append(line);
        }
    }
}
