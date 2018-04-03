extern crate flashlight;
extern crate isatty;
extern crate regex;

use isatty::stdin_isatty;
use std::io;
use flashlight::Runner;
use flashlight::LineReader;

fn main() {
    let args = flashlight::Args::build();
    let config = flashlight::Config::from(&args);

    let runner = if let &Some(ref filename) = &args.flag_log.clone() {
        Runner { reader: LineReader::file(filename, config.tail)}
    } else if stdin_isatty() {
        Runner { reader: LineReader::file("log/development.log", config.tail)}
    } else {
        Runner { reader: LineReader::stdin()}
    };

    runner.run(config, &mut io::stdout());
}
