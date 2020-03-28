mod parser;
pub mod tree;
pub mod types;
use parser::{parse, tokenize};
use std::fs;
use tree::Tree;
use types::{ParseError, ParseResult};

/// Parse a json text:
///
/// **Example**
/// ```
/// let text = "{ \"foo\": \"bar\" }";
/// match parse_json(&text) {
///     Ok(tree) => println!("{:?}", tree),
///     Err(e) => println!("{:?}", e),
/// };
/// ```
pub fn parse_json(text: &str) -> ParseResult<Tree> {
    let mut vec = Vec::new();
    tokenize::tokenize(&text, &mut vec);
    parse::parse_json(&vec[..])
}

/// Parse a json file:
///
/// **Example**
/// ```
/// let file_name = "foo.json";
/// match parse_json_file(&file_name) {
///     Ok(tree) => println!("{:?}", tree),
///     Err(e) => println!("{:?}", e),
/// };
/// ```
pub fn parse_json_file(file_path: &str) -> ParseResult<Tree> {
    match fs::read_to_string(file_path) {
        Ok(text) => {
            let mut vec = Vec::new();
            tokenize::tokenize(&text, &mut vec);
            parse::parse_json(&vec[..])
        }
        Err(_) => Err(ParseError::FileNotFound),
    }
}

