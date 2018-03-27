use docopt::Docopt;
use super::*;

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

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_pattern: String,
    pub flag_file: Option<String>,
    pub flag_buffer_size: usize,
    pub flag_web: bool,
    pub flag_cable: bool,
    pub flag_jobs: bool,
    pub flag_assets: bool,
}

impl Args {
    pub fn build() -> Self {
        Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
    }
}

pub struct Config {
    pub buffer_size: usize,
    pub contexts: HashSet<Context>,
    pub matcher: Regex,
}

impl Config {
    pub fn default() -> Self {
        Config {
            contexts: HashSet::new(),
            matcher: Regex::new(&"".to_string()).unwrap(),
            buffer_size: 10_000,
        }
    }

    pub fn matching(mut self, string: &str) -> Self {
        self.matcher = Regex::new(string).unwrap();
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

        let matcher = Regex::new(&regex::escape(&args.flag_pattern)).unwrap();

        Config {
            contexts,
            matcher,
            buffer_size: args.flag_buffer_size,
        }
    }
}
