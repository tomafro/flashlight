use super::*;
use docopt::Docopt;
use std::collections::HashSet;

const USAGE: &'static str = "
Flashlight.

Usage:
  flashlight [options] [<string>...]
  flashlight -h | --help

Options:
  -h --help                 Show this screen.
  --web                     Show logging from web requests
  --cable                   Show logging from ActionCable
  --jobs                    Show logging from ActiveJob
  --assets                  Show logging from assets
  --log <log>               Log file (defaults to ./log/development.log)
  --tail                    Tail the log file
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub arg_string: Vec<String>,
    pub flag_log: Option<String>,
    pub flag_web: bool,
    pub flag_cable: bool,
    pub flag_jobs: bool,
    pub flag_assets: bool,
    pub flag_tail: bool,
}

impl Args {
    pub fn build() -> Self {
        Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
    }
}

#[derive(Debug)]
pub struct Config {
    pub buffer_size: usize,
    pub tail: bool,
    pub matcher: Matcher,
    pub filter: Matcher,
}

impl Config {
    pub fn default() -> Self {
        Config {
            filter: Matcher::Everything,
            matcher: Matcher::Everything,
            buffer_size: 10_000,
            tail: false,
        }
    }

    pub fn matching(mut self, string: &str) -> Self {
        self.matcher = Matcher::match_strings(&vec![string.to_string()]);
        self
    }

    pub fn match_contexts(mut self, contexts: Vec<Context>) -> Self {
        let mut new_contexts = HashSet::new();
        for c in contexts {
            new_contexts.insert(c);
        }
        if new_contexts.is_empty() {
            self.filter = Matcher::Everything;
        } else {
            self.filter = Matcher::ContextsMatcher(new_contexts);
        }
        self
    }
}

impl<'a> From<&'a Args> for Config {
    fn from(args: &Args) -> Self {
        let mut contexts = HashSet::new();

        if args.flag_assets {
            contexts.insert(Context::Asset);
        }
        if args.flag_web {
            contexts.insert(Context::Web);
        }
        if args.flag_cable {
            contexts.insert(Context::Cable);
        }
        if args.flag_jobs {
            contexts.insert(Context::Job);
        }

        let mut filter = Matcher::Everything;

        if !contexts.is_empty() {
            filter = Matcher::ContextsMatcher(contexts)
        }

        let mut matcher = Matcher::Everything;

        if !args.arg_string.is_empty() {
            matcher = Matcher::match_strings(&args.arg_string);
        }

        Config {
            filter,
            matcher,
            buffer_size: 10_000,
            tail: args.flag_tail,
        }
    }
}
