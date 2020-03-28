use std::{error, fmt, ops::Add};

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingObjectBrace,
    MissingArrayBrace,
    InvalidType,
    FileNotFound,
    Error,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidType => write!(f, "Found invalid type"),
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

impl Range {
    pub fn new(start: Position, end: Position) -> Range {
        Range { start, end }
    }
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
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            line: self.line + other.line,
            char: self.char + other.char,
            idx: self.idx + other.idx,
        }
    }
}
