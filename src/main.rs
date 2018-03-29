extern crate flashlight;
extern crate isatty;
extern crate regex;

use std::io;
use isatty::stdin_isatty;

fn main() {
    let args = flashlight::Args::build();
    let config = flashlight::Config::from(&args);

    if let &Some(ref filename) = &args.flag_log.clone() {
        flashlight::run_with_file(config, filename, &mut io::stdout());
    } else if stdin_isatty() {
        flashlight::run_with_file(config, "log/development.log", &mut io::stdout());
    } else {
        flashlight::run_with_stdin(config, &mut io::stdout());
    };
}
