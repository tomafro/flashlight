use super::*;
use regex::RegexSet;

#[derive(Debug)]
pub enum Matcher {
    RegexMatcher(RegexSet),
    Everything,
}

impl Matcher {
    pub fn for_strings(strings: &Vec<String>) -> Matcher {
        Matcher::RegexMatcher(RegexSet::new(strings).unwrap())
    }

    pub fn matches(&self, line: &Line) -> bool {
        match self {
            Matcher::RegexMatcher(set) => set.is_match(line.content()),
            Matcher::Everything => true,
        }
    }
}
