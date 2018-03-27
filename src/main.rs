extern crate flashlight;
extern crate regex;

use std::io::{self, BufReader};
use std::fs::File;

fn main() {
    let args = flashlight::Args::build();
    let config = flashlight::Config::from(&args);

    if let &Some(ref filename) = &args.flag_file.clone() {
        let file = File::open(filename).expect("File not found");
        flashlight::run(config, BufReader::new(file), &mut io::stdout());
    } else {
        let stdin = io::stdin();
        flashlight::run(config, stdin.lock(), &mut io::stdout());
    };
}
