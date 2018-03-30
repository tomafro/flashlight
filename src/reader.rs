use std::{thread, time};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::io::BufReader;
use std::io::{Seek,SeekFrom};

use super::Line;

// pub struct LineReader<'a> {
//     reader: &'a (BufRead + 'a), // with parens
// }

// impl<'a> LineReader<'a> {
//     pub fn <T>from(input: T) -> LineReader
//     where T: BufRead {
//         LineReader { reader: &stdin.lock() }
//     }
// }

pub struct FileReader {
    tail: bool,
    reader: BufReader<File>
}

impl FileReader {
    pub fn new(path: String, tail: bool) -> FileReader {
        let file = File::open(&path).unwrap();
        let mut reader = BufReader::new(file);

        if tail {
            let metadata = fs::metadata(&path).unwrap();
            let offset = metadata.len() - 1000;

            reader.seek(SeekFrom::Start(offset)).unwrap();
            reader.read_line(&mut String::new()).unwrap();
        }

        FileReader { reader, tail }
    }
}

impl Iterator for FileReader {
    type Item = Line;

    fn next(&mut self) -> Option<Line> {
        let pause = time::Duration::from_millis(100);
        let mut line = String::new();

        loop {
            if let Ok(bytes) = self.reader.read_line(&mut line) {
                if bytes > 0 {
                    return Some(Line::from(line))
                }
                else {
                    if (self.tail) {
                        thread::sleep(pause);
                    }
                    else {
                        return None
                    }
                }
            }
            else {
                return None
            }
        }
    }
}

