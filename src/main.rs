extern crate flashlight;

use std::io;
use flashlight::Runner;

fn main() {
    let runner = Runner::from_cli();
    runner.run(&mut io::stdout());
}
