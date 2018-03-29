# flashlight

Flashlight is an experimental tool written in rust to help spelunking Basecamp 3 logs. It searches logs for a specific string, and outputs **the entire request** that string was found in. For example
```
cat dev.log | flashlight 'my/readings/unreads/index'
```

It also supports cutting down the logs to show only specific contexts, so to show only logging from web requests and jobs:
```
cat dev.log | flashlight --web --jobs
```

It will work on any input piped into it, or you can specify a log file using `--log`. If no file is specified, and nothing is piped in, it defaults to using the file `log/development.log`.

Finally you can tail logs too, with `--tail`.

As an example putting all of this together, when run from the BC3 directory the following will tail `log/development.log`, printing out all web requests matching "Tom Ward":
```
flashlight --web --tail "Tom Ward"
```

## Building

* Clone this repo
* Install [rustup](https://www.rustup.rs/)
* In the repo root, run: `rustup run nightly cargo build --release`

The flashlight binary should now be built in `target/release/flashlight`

## Caveat emptor

I've written this partly to scratch an itch, but mainly to play around with rust. The code works, but is terrible. Use at your own risk.
