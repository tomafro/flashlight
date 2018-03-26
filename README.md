# flashlight

Flashlight is an experimental tool written in rust to help spelunking Basecamp 3 logs. It searches logs for a specific string, and outputs **the entire request** that string was found in. For example 
```
cat dev.log | flashlight --pattern 'my/readings/unreads/index'
```

It also supports cutting down the logs to show only specific contexts, so to show only logging from web requests and jobs:
```
cat dev.log | flashlight --web --jobs
```

## Building

* Clone this repo
* Install [rustup](https://www.rustup.rs/)
* In the repo root, run: `rustup run nightly cargo build --release`

The flashlight binary should now be built in `target/release/flashlight`

## Caveat emptor

I've written this partly to scratch an itch, but mainly to play around with rust. The code works, but is terrible. Use at your own risk.
