#![feature(non_modrs_mods)]
#![feature(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate indoc;

extern crate regex;

mod buffer;
mod parser;
mod test;

pub use parser::Line;
pub use buffer::Buffer;
pub use parser::Context;

use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};

#[derive(Debug, Deserialize)]
pub struct Args {
    flag_pattern: String,
    pub flag_file: Option<String>,
    flag_buffer_size: usize,
    flag_web: bool,
    flag_cable: bool,
    flag_jobs: bool,
    flag_assets: bool
}

pub fn run<T: BufRead, U: Write>(args: Args, input: T, output: &mut U) {
    let matcher = Regex::new(&regex::escape(&args.flag_pattern)).unwrap();
    let mut buffer = Buffer::new(args.flag_buffer_size);
    let mut matched_requests = HashSet::new();

    for l in input.lines() {
        let line = Line::from(l.unwrap());

        if allow_line(&line, &args) {
            if matched_requests.contains(line.request_id()) {
                write!(output, "{}\n", line.content()).unwrap();
            }
            else if matcher.is_match(line.content()) {
                for previous in buffer.lines().into_iter().filter(|l| l.request_id() == line.request_id()) {
                    write!(output, "{}\n", previous.content()).unwrap();
                }
                write!(output, "{}\n", line.content()).unwrap();
                matched_requests.insert(line.request_id().clone());
            }
            buffer.append(line);
        }
    }
}

fn allow_line(line: &Line, args: &Args) -> bool {
    if args.flag_cable || args.flag_jobs || args.flag_web || args.flag_assets {
        (args.flag_cable && line.context() == &Context::Cable) || (args.flag_jobs && line.context() == &Context::Job) || (args.flag_web && line.context() == &Context::Web) || (args.flag_assets && line.context() == &Context::Asset)
    }
    else {
        true
    }
}
