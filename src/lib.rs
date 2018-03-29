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
mod matcher;
#[cfg(test)]
mod test;

pub use cli::Args;
pub use cli::Config;
pub use parser::Line;
pub use buffer::Buffer;
pub use parser::Context;
pub use matcher::Matcher;

use std::collections::HashSet;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::fs;
use std::{thread, time};
use std::io::{Seek,SeekFrom};


pub fn run_with_stdin<U: Write>(config: Config, output: &mut U) {
    let stdin = io::stdin();
    run(config, stdin.lock(), output);
}


pub fn run_with_file<U: Write>(config: Config, filename: &str, output: &mut U) {
    let file = File::open(filename).expect("File not found");

    let mut reader = BufReader::new(file);

    let metadata = fs::metadata("/Users/tom/Work/basecamp/bc3/log/development.log").unwrap();
    let offset = metadata.len() - 1000;
    reader.seek(SeekFrom::Start(offset)).unwrap();
    let mut _string = String::new();
    reader.read_line(&mut _string).unwrap();
    run(config, reader, output);
}

pub fn run<T: BufRead, U: Write>(config: Config, mut input: T, output: &mut U) {
    let mut buffer = Buffer::new(config.buffer_size);
    let mut matched_requests = HashSet::new();
    let mut input_line = String::new();

    println!("{:?}", config);

    while let Ok(bytes) = input.read_line(&mut input_line) {
        if bytes > 0 {
            let cloned = input_line.clone();
            let line = Line::from(cloned);
            if config.contexts.is_empty() || config.contexts.contains(line.context()) {
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
        else {
            if config.tail {
                let hundred_millis = time::Duration::from_millis(100);
                thread::sleep(hundred_millis);
            } else {
                break;
            }
        }
        input_line.clear();
    }
}
