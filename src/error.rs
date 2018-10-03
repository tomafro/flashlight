use std::fmt;

pub struct FileNotFoundError {
    path: String
}

impl FileNotFoundError {
    pub fn new(path: &str) -> FileNotFoundError {
        FileNotFoundError{path: path.to_string()}
    }
}
impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Log file '{}' wasn't found", self.path)
    }
}
