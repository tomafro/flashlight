use isatty::stdin_isatty;
use std::collections::HashSet;
use std::io::Write;

use super::*;

pub struct Runner {
    pub matcher: Matcher,
    pub filter: Matcher,
    pub reader: LineReader,
    pub buffer_size: usize,
}

impl Runner {
    pub fn from_cli() -> Runner {
        let args = Args::build();
        let Config {
            tail,
            matcher,
            filter,
            buffer_size,
        } = Config::from(&args);

        let reader = if let &Some(ref filename) = &args.flag_log.clone() {
            LineReader::file(filename, tail)
        } else if stdin_isatty() {
            LineReader::file("log/development.log", tail)
        } else {
            LineReader::stdin()
        };

        Runner {
            filter,
            matcher,
            buffer_size,
            reader,
        }
    }
}

impl Runner {
    pub fn run<U: Write>(self, output: &mut U) {
        let mut buffer = Buffer::new(self.buffer_size);
        let mut matched_requests = HashSet::new();
        let filter = self.filter;

        for line in self.reader
            .map(|l| Line::from(l))
            .filter(|l| filter.matches(&l))
        {
            if matched_requests.contains(line.request_id()) {
                write!(output, "{}", line.content()).unwrap();
            } else if self.matcher.matches(&line) {
                if line.request_id().is_some() {
                    for previous in buffer
                        .lines()
                        .into_iter()
                        .filter(|l| l.request_id() == line.request_id())
                    {
                        write!(output, "{}", previous.content()).unwrap();
                    }
                    matched_requests.insert(line.request_id().clone());
                }
                write!(output, "{}", line.content()).unwrap();
            }
            buffer.append(line);
        }
    }
}
