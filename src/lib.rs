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

mod buffer;
mod cli;
mod matcher;
mod parser;
mod reader;
#[cfg(test)]
mod test;

pub use buffer::Buffer;
pub use cli::Args;
pub use cli::Config;
pub use matcher::Matcher;
pub use parser::Context;
pub use parser::Line;
pub use reader::LineReader;

use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::io::{Seek, SeekFrom};
use std::{thread, time};

pub struct Runner {
    pub reader: LineReader,
}

impl Runner {
    pub fn run<U: Write>(self, config: Config, output: &mut U) {
        let mut buffer = Buffer::new(config.buffer_size);
        let mut matched_requests = HashSet::new();

        for line in self.reader.map(|l| Line::from(l)).filter(|l| config.filter.matches(&l)) {
            if matched_requests.contains(line.request_id()) {
                write!(output, "{}", line.content()).unwrap();
            } else if config.matcher.matches(&line) {
                if line.request_id().is_some() {
                    for previous in buffer
                        .lines()
                        .into_iter()
                        .filter(|l| l.request_id() == line.request_id())
                    {
                        write!(output, "{}", previous.content()).unwrap();
                    }
                    matched_requests.insert(line.request_id().clone());
                }
                write!(output, "{}", line.content()).unwrap();
            }
            buffer.append(line);
        }
    }
}
