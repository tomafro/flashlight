extern crate regex;
extern crate flashlight;

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

use flashlight::*;

use docopt::Docopt;

const USAGE: &'static str = "
Hacksaw.

Usage:
  hacksaw [--pattern=<pattern>] [--web] [--cable] [--assets] [--jobs] [--buffer-size=<size>]
  hacksaw -h | --help

Options:
  -h --help                 Show this screen.
  --version                 Show version.
  --web                     Show web context
  --buffer-size=<size>      Buffer size [default: 10000].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_pattern: String,
    arg_file: Option<String>,
    flag_buffer_size: usize,
    flag_web: bool,
    flag_cable: bool,
    flag_jobs: bool,
    flag_assets: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    run(Box::new(io::stdout()), args);
}

fn run(mut output: Box<Write>, args: Args) {
    let matcher = Regex::new(&regex::escape(&args.flag_pattern)).unwrap();
    let stdin = io::stdin();
    let mut buffer = Buffer::new(args.flag_buffer_size);

    let input: Box<BufRead> = if let &Some(ref filename) = &args.arg_file {
        Box::new(BufReader::new(File::open(filename).unwrap()))
    }
    else {
        Box::new(stdin.lock())
    };

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
        (args.flag_cable && line.context() == &flashlight::Context::Cable) || (args.flag_jobs && line.context() == &flashlight::Context::Job) || (args.flag_web && line.context() == &flashlight::Context::Web) || (args.flag_assets && line.context() == &flashlight::Context::Asset)
    }
    else {
        true
    }
}
