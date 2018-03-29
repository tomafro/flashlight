use regex::RegexSet;
use super::*;

#[derive(Debug)]
pub enum Matcher {
    RegexMatcher(RegexSet),
    Everything
}

impl Matcher {
  pub fn matches(&self, line: &Line) -> bool {
      match self {
          Matcher::RegexMatcher(set) => set.is_match(line.content()),
          Matcher::Everything => true
      }
  }
}
// pub struct RegexMatcher {
//     regex: RegexSet,
// }

// impl RegexMatcher {
//     fn new(strings: &Vec<String>) -> RegexMatcher {
//         RegexMatcher { regex: RegexSet::new(strings).unwrap() }
//     }
// }

// impl Matcher for RegexMatcher {
//     fn matches(&self, line: &Line) -> bool {
//         self.regex.is_match(line.content())
//     }
// }