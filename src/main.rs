extern crate regex;
extern crate flashlight;
extern crate docopt;

use docopt::Docopt;
use std::io::{self, BufReader};
use flashlight::Args;
use std::fs::File;


const USAGE: &'static str = "
Flashlight.

Usage:
  flashlight [--pattern=<pattern>] [--web] [--cable] [--assets] [--jobs] [--buffer-size=<size>] [--file=<file>]
  flashlight -h | --help

Options:
  -h --help                 Show this screen.
  --version                 Show version.
  --web                     Show web context
  --buffer-size=<size>      Buffer size [default: 10000].
";

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    if let &Some(ref filename) = &args.flag_file.clone() {
        let file = File::open(filename).expect("File not found");
        flashlight::run(args, BufReader::new(file), &mut io::stdout());
    }
    else {
        let stdin = io::stdin();
        flashlight::run(args, stdin.lock(), &mut io::stdout());
    };
}