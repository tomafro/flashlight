use docopt::Docopt;
use super::*;

const USAGE: &'static str = "
Flashlight.

Usage:
  flashlight [options] [<string>...]
  flashlight -h | --help

Options:
  -h --help                 Show this screen.
  --version                 Show version.
  --web                     Show logging from web requests
  --cable                   Show logging from ActionCable
  --jobs                    Show logging from ActiveJob
  --assets                  Show logging from assets
  --log                     Log file (defaults to ./log/development.log)
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
    pub contexts: HashSet<Context>,
    pub matcher: RegexSet,
    pub tail: bool
}

impl Config {
    pub fn default() -> Self {
        Config {
            contexts: HashSet::new(),
            matcher: RegexSet::new(&[""]).unwrap(),
            buffer_size: 10_000,
            tail: false
        }
    }

    pub fn matching(mut self, string: &str) -> Self {
        self.matcher = RegexSet::new(&[string]).unwrap();
        self
    }

    pub fn match_contexts(mut self, contexts: Vec<Context>) -> Self {
        let mut new_contexts = HashSet::new();
        for c in contexts {
            new_contexts.insert(c);
        }
        self.contexts = new_contexts;
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

        let mut strings = &vec!["".to_string()];

        if !args.arg_string.is_empty() {
            strings = &args.arg_string;
        }

        let matcher = RegexSet::new(strings).unwrap();

        Config {
            contexts,
            matcher,
            buffer_size: 10_000,
            tail: args.flag_tail
        }
    }
}
