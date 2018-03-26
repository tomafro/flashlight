use super::*;
use std::collections::VecDeque;

pub struct Buffer {
    pub max_size: usize,

    buffer: VecDeque<Line>,
    line_number: usize,
}

impl Buffer {
    pub fn new(max_size: usize) -> Buffer {
        let buffer = VecDeque::with_capacity(max_size + 1);
        Buffer { max_size, buffer, line_number: 0 }
    }

    pub fn append(&mut self, line: Line) {
        self.buffer.push_back(line);
        self.line_number = self.line_number + 1;

        if self.buffer.len() > self.max_size {
            self.buffer.pop_front();
        }
    }

    pub fn lines(&self) -> &VecDeque<Line> {
      &self.buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn limited_to_given_size() {
        let mut buffer = Buffer::new(3);
        assert_eq!(0, buffer.lines().len());

        buffer.append(Line::from("[A] [abc12…]"));
        assert_eq!(1, buffer.lines().len());

        buffer.append(Line::from("[B] [abc12…]"));
        buffer.append(Line::from("[C] [abc12…]"));
        buffer.append(Line::from("[D] [abc12…]"));
        buffer.append(Line::from("[E] [abc12…]"));

        assert_eq!(3, buffer.lines().len());
    }
}

