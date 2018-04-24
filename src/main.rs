extern crate flashlight;

use flashlight::Runner;
use std::io;

fn main() {
    let runner = Runner::from_cli();
    runner.run(&mut io::stdout());
}
