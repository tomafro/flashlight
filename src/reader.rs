use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::{thread, time};

use error::FileNotFoundError;

pub struct LineReader {
    pub input: Box<BufRead>,
    pub tail: bool,
}

impl LineReader {
    pub fn stdin() -> Result<LineReader, FileNotFoundError> {
        let input = Box::new(BufReader::new(io::stdin()));
        let tail = false;
        Ok(LineReader { input, tail })
    }

    pub fn file(path: &str, tail: bool) -> Result<LineReader, FileNotFoundError> {
        if let Ok(file) = File::open(path) {
            let mut reader = BufReader::new(file);

            if tail {
                let metadata = fs::metadata(&path).unwrap();
                let offset = metadata.len() - 1000;

                reader.seek(SeekFrom::Start(offset)).unwrap();
                reader.read_line(&mut String::new()).unwrap();
            }

            let input = Box::new(reader);
            Ok(LineReader { input, tail })
        } else {
            Err(FileNotFoundError::new(path))
        }
    }

    pub fn string(string: &'static str) -> LineReader {
        let input = Box::new(BufReader::new(string.as_bytes()));
        let tail = false;
        LineReader { input, tail }
    }
}

impl Iterator for LineReader {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let pause = time::Duration::from_millis(100);
        let mut line = String::new();

        loop {
            if let Ok(bytes) = self.input.read_line(&mut line) {
                if bytes > 0 {
                    return Some(line);
                } else {
                    if self.tail {
                        thread::sleep(pause);
                    } else {
                        return None;
                    }
                }
            } else {
                return None;
            }
        }
    }
}
