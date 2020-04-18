mod parser;
pub mod tree;
pub mod types;
use parser::{parse, tokenize};
use std::fs;
use tree::Tree;
use tokenize::TokenType;
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
    tokenize::tokenize(&text)
        .map(|tokens| tokens)
        .and_then(|tokens| {

            let filtered_tokens: Vec<TokenType> = tokens.into_iter().filter(|e| {
                if let TokenType::Comment(_,_) = e {
                    return false;
                }
        
                true
            }).collect();
            
            parse::parse_json(&filtered_tokens[..])
        })
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
        Ok(text) => parse_json(&text),
        Err(_) => Err(ParseError::FileNotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::tree::{EntryType, PathType};
    #[test]
    fn test_parse() {
        let json = "// hello\n { \n // this is a test\n \"a\": {}, \"b\": { \"c\": [true, { \"e\": 42 } ] } } // bli \n// bla";
        match super::parse_json(json) {
            Ok(tree) => {
                let res = tree.value_at(&[PathType::Object("a")]);
                let entry = res.get(0).unwrap();
                match (*(*entry)).entry_type {
                    EntryType::JSONObject(_) => println!("Correct entry"),
                    _ => panic!("Should be object"),
                }

                let res = tree.value_at(&[
                    PathType::Object("b"),
                    PathType::Object("c"),
                    PathType::Array(1),
                    PathType::Object("e"),
                ]);
                let entry = res.get(0).unwrap();
                match (*(*entry)).entry_type {
                    EntryType::Int(val) => assert_eq!(42, val),
                    _ => panic!("Should be number"),
                }

                let res = tree.value_at(&[
                    PathType::Wildcard,
                    PathType::Object("c"),
                    PathType::Array(0),
                ]);
                let entry = res.get(0).unwrap();
                match (*(*entry)).entry_type {
                    EntryType::Bool(val) => assert_eq!(true, val),
                    _ => panic!("Should be bool"),
                }

                let res = tree.value_at(&[
                    PathType::RecursiveWildcard,
                    PathType::Array(1),
                    PathType::Object("e"),
                ]);
                let entry = res.get(0).unwrap();
                match (*(*entry)).entry_type {
                    EntryType::Int(val) => assert_eq!(42, val),
                    _ => panic!("Should be number"),
                }

                let res = tree.value_at(&[
                    PathType::Wildcard,
                    PathType::Object("c"),
                    PathType::Array(1),
                    PathType::Object("e"),
                ]);
                let entry = res.get(0).unwrap();
                match (*(*entry)).entry_type {
                    EntryType::Int(val) => assert_eq!(42, val),
                    _ => panic!("Should be number"),
                }
            }
            Err(_) => panic!("Could not parse json."),
        }
    }
}
