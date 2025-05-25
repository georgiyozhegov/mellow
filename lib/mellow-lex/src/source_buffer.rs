pub struct SourceBuffer {
    buffer: Vec<char>,
    current: usize,
}

impl From<String> for SourceBuffer {
    fn from(buffer: String) -> Self {
        let buffer = buffer.chars().collect();
        Self::new(buffer)
    }
}

impl From<&str> for SourceBuffer {
    fn from(buffer: &str) -> Self {
        let buffer = buffer.chars().collect();
        Self::new(buffer)
    }
}

impl SourceBuffer {
    pub fn new(buffer: Vec<char>) -> Self {
        Self { buffer, current: 0 }
    }

    pub fn eat(&mut self) -> char {
        let c = self.current().unwrap();
        self.advance();
        c
    }

    pub fn take_while(&mut self, pattern: fn(char) -> bool) -> String {
        let mut output = String::new();
        while self.current_is(pattern) {
            output.push(self.eat());
        }
        output
    }

    pub fn current_is(&self, f: fn(char) -> bool) -> bool {
        self.current().is_some_and(f)
    }

    pub fn current(&self) -> Option<char> {
        self.buffer.get(self.current).cloned()
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }
}

pub fn is_alphanumeric(c: char) -> bool {
    is_alphabetic(c) | is_numeric(c)
}

pub fn is_numeric(c: char) -> bool {
    matches!(c, '0'..='9')
}

pub fn is_alphabetic(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_skip(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}
