use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Seek, SeekFrom};
use std::{thread, time};
use std::collections::HashSet;
use isatty::stdin_isatty;

use super::*;

pub struct Runner {
    pub config: Config,
    pub reader: LineReader,
}

impl Runner {
    pub fn from_cli() -> Runner {
        let args = Args::build();
        let config = Config::from(&args);
        let tail = config.tail;

        if let &Some(ref filename) = &args.flag_log.clone() {
            Runner { config, reader: LineReader::file(filename, tail)}
        } else if stdin_isatty() {
            Runner { config, reader: LineReader::file("log/development.log", tail)}
        } else {
            Runner { config, reader: LineReader::stdin()}
        }
    }
}

impl Runner {
    pub fn run<U: Write>(self, output: &mut U) {
        let mut buffer = Buffer::new(self.config.buffer_size);
        let mut matched_requests = HashSet::new();
        let filter = self.config.filter;

        for line in self.reader.map(|l| Line::from(l)).filter(|l| filter.matches(&l)) {
            if matched_requests.contains(line.request_id()) {
                write!(output, "{}", line.content()).unwrap();
            } else if self.config.matcher.matches(&line) {
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
