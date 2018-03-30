use super::*;
use regex::RegexSet;

#[derive(Debug)]
pub enum Matcher {
    RegexMatcher(RegexSet),
    ContextsMatcher(HashSet<Context>),
    Everything,
}

impl Matcher {
    pub fn match_strings<I, S>(collection: I) -> Matcher
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        Matcher::RegexMatcher(RegexSet::new(collection).unwrap())
    }

    pub fn matches(&self, line: &Line) -> bool {
        match self {
            Matcher::RegexMatcher(regex) => regex.is_match(line.content()),
            Matcher::ContextsMatcher(set) => set.contains(line.context()),
            Matcher::Everything => true,
        }
    }
}
