use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingObjectBrace,
    MissingArrayBrace,
    FileNotFound,
    Error,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingArrayBrace => write!(f, "Missing array brace"),
            ParseError::MissingObjectBrace => write!(f, "Missing object brace"),
            ParseError::FileNotFound => write!(f, "File not found"),
            ParseError::Error => write!(f, "Could not parse json"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub char: usize,
    pub idx: usize,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            line: 0,
            char: 0,
            idx: 0,
        }
    }
}

impl Position {
    pub fn new(line: usize, char: usize, idx: usize) -> Position {
        Position { line, char, idx }
    }

    pub fn add(&self, line: usize, char: usize) -> Position {
        Position {
            line: self.line + line,
            char: self.char + char,
            idx: self.idx + char,
        }
    }
}