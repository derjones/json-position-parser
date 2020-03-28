use super::types::{ParseError, ParseResult, Position, Range};

enum CurrentTokenType {
    String,
    Number,
    Bool,
    Null,
}

#[derive(Debug)]
pub enum TokenType {
    String(Range, String),
    Float(Range, f64),
    Int(Range, i64),
    Bool(Range, bool),
    Null(Range),
    ObjectOpen(Range),
    ObjectClose(Range),
    ArrayOpen(Range),
    ArrayClose(Range),
    Comma(Range),
    Semicolon(Range),
}

fn handle_defaults(c: char, tokens: &mut Vec<TokenType>, pos: Position) {
    match c {
        '{' => {
            tokens.push(TokenType::ObjectOpen(Range {
                start: pos,
                end: pos + Position::new(0, 1, 1),
            }));
        }
        '}' => {
            tokens.push(TokenType::ObjectClose(Range {
                start: pos,
                end: pos + Position::new(0, 1, 1),
            }));
        }
        '[' => {
            tokens.push(TokenType::ArrayOpen(Range {
                start: pos,
                end: pos + Position::new(0, 1, 1),
            }));
        }
        ']' => {
            tokens.push(TokenType::ArrayClose(Range {
                start: pos,
                end: pos + Position::new(0, 1, 1),
            }));
        }
        ',' => {
            tokens.push(TokenType::Comma(Range {
                start: pos,
                end: pos + Position::new(0, 1, 1),
            }));
        }
        ':' => {
            tokens.push(TokenType::Semicolon(Range {
                start: pos,
                end: pos + Position::new(0, 1, 1),
            }));
        }
        _ => {}
    };
}

pub fn tokenize(string: &str) -> ParseResult<Vec<TokenType>> {
    let mut tokens = vec![];
    let mut current_type = None;
    let mut current_type_start = Position::default();
    let mut concat_string = String::new();
    let mut current_line = 0;
    let mut current_char = 0;
    let mut escaped = false;

    string
        .chars()
        .enumerate()
        .map(|(pos, c)| {
            if let '\n' = c {
                current_line += 1;
                current_char = 0;
                return Ok(());
            }
            match &current_type {
                Some(t) => match t {
                    CurrentTokenType::String => {
                        if !escaped && c == '\\' {
                            escaped = true;
                            concat_string.push(c);
                        } else if !escaped && c == '"' {
                            tokens.push(TokenType::String(
                                Range {
                                    start: current_type_start + Position::new(0, 1, 1),
                                    end: Position::new(current_line, current_char, pos),
                                },
                                concat_string.to_owned(),
                            ));
                            current_type = None;
                            concat_string = String::new();
                        } else {
                            escaped = false;
                            concat_string.push(c);
                        }
                        ParseResult::Ok(())
                    }
                    CurrentTokenType::Number => {
                        if !c.is_digit(10) && c != '.' {
                            if let Ok(int) = concat_string.parse::<i64>() {
                                tokens.push(TokenType::Int(
                                    Range {
                                        start: current_type_start,
                                        end: Position::new(current_line, current_char, pos),
                                    },
                                    int,
                                ));
                            } else if let Ok(float) = concat_string.parse::<f64>() {
                                tokens.push(TokenType::Float(
                                    Range {
                                        start: current_type_start,
                                        end: Position::new(current_line, current_char, pos),
                                    },
                                    float,
                                ));
                            } else {
                                return Err(ParseError::InvalidType);
                            }
                            current_type = None;
                            concat_string = String::new();
                            handle_defaults(
                                c,
                                &mut tokens,
                                Position::new(current_line, current_char, pos),
                            );
                        } else {
                            concat_string.push(c);
                        }

                        Ok(())
                    }
                    CurrentTokenType::Bool => {
                        if concat_string == "true" || concat_string == "false" {
                            tokens.push(TokenType::Bool(
                                Range {
                                    start: current_type_start,
                                    end: Position::new(current_line, current_char, pos),
                                },
                                concat_string.parse::<bool>().unwrap(),
                            ));
                            current_type = None;
                            concat_string = String::new();
                            handle_defaults(
                                c,
                                &mut tokens,
                                Position::new(current_line, current_char, pos),
                            );
                        } else if !"true".starts_with(&concat_string) && !"false".starts_with(&concat_string) {
                            return Err(ParseError::InvalidType);
                        }else {
                            concat_string.push(c);
                        }

                        Ok(())
                    }
                    CurrentTokenType::Null => {
                        if concat_string == "null" {
                            tokens.push(TokenType::Null(Range {
                                start: current_type_start,
                                end: Position::new(current_line, current_char, pos),
                            }));
                            current_type = None;
                            concat_string = String::new();
                            handle_defaults(
                                c,
                                &mut tokens,
                                Position::new(current_line, current_char, pos),
                            );
                        } else if !"null".starts_with(&concat_string) {
                            return Err(ParseError::InvalidType);
                        } else {
                            concat_string.push(c);
                        }

                        Ok(())
                    }
                },
                None => match c {
                    '"' => {
                        current_type = Some(CurrentTokenType::String);
                        current_type_start = Position::new(current_line, current_char, pos);
                        Ok(())
                    }
                    'n' => {
                        current_type = Some(CurrentTokenType::Null);
                        current_type_start = Position::new(current_line, current_char, pos);
                        concat_string.push(c);
                        Ok(())
                    }
                    't' | 'f' => {
                        current_type = Some(CurrentTokenType::Bool);
                        current_type_start = Position::new(current_line, current_char, pos);
                        concat_string.push(c);
                        Ok(())
                    }
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        current_type = Some(CurrentTokenType::Number);
                        current_type_start = Position::new(current_line, current_char, pos);
                        concat_string.push(c);
                        Ok(())
                    }
                    _ => {
                        handle_defaults(
                            c,
                            &mut tokens,
                            Position::new(current_line, current_char, pos),
                        );
                        Ok(())
                    }
                },
            }
            .ok()
            .map(|_| current_char += 1)
            .ok_or(ParseError::InvalidType)
        })
        .collect::<Result<Vec<()>, ParseError>>()
        .map(|_| (tokens))
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Position, Range, TokenType};
    use float_cmp::approx_eq;

    #[test]
    fn test_tokenize() {
        let primitives = "{ \"string\": \"value\", \"null\": null, \"bool1\": false, \"bool2\": true, \"int\": 1, \"float\": 1.0, \"array\": [] }";
        match tokenize(primitives) {
            Ok(tokens) => {
                equal_token_single(
                    tokens.get(0).unwrap(),
                    &TokenType::ObjectOpen(Range::new(
                        Position::new(0, 0, 0),
                        Position::new(0, 1, 1),
                    )),
                );

                equal_token_tuple(
                    tokens.get(1).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 3, 3), Position::new(0, 9, 9)),
                        "string".to_owned(),
                    ),
                );

                equal_token_tuple(
                    tokens.get(3).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 13, 13), Position::new(0, 18, 18)),
                        "value".to_owned(),
                    ),
                );

                equal_token_tuple(
                    tokens.get(5).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 22, 22), Position::new(0, 26, 26)),
                        "null".to_owned(),
                    ),
                );

                equal_token_single(
                    tokens.get(7).unwrap(),
                    &TokenType::Null(Range::new(
                        Position::new(0, 29, 29),
                        Position::new(0, 33, 33),
                    )),
                );

                equal_token_tuple(
                    tokens.get(9).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 36, 36), Position::new(0, 41, 41)),
                        "bool1".to_owned(),
                    ),
                );

                equal_token_tuple(
                    tokens.get(11).unwrap(),
                    &TokenType::Bool(
                        Range::new(Position::new(0, 44, 44), Position::new(0, 49, 49)),
                        false,
                    ),
                );

                equal_token_tuple(
                    tokens.get(13).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 52, 52), Position::new(0, 57, 57)),
                        "bool2".to_owned(),
                    ),
                );

                equal_token_tuple(
                    tokens.get(15).unwrap(),
                    &TokenType::Bool(
                        Range::new(Position::new(0, 60, 60), Position::new(0, 64, 64)),
                        true,
                    ),
                );

                equal_token_tuple(
                    tokens.get(17).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 67, 67), Position::new(0, 70, 70)),
                        "int".to_owned(),
                    ),
                );

                equal_token_tuple(
                    tokens.get(19).unwrap(),
                    &TokenType::Int(
                        Range::new(Position::new(0, 73, 73), Position::new(0, 74, 74)),
                        1,
                    ),
                );

                equal_token_tuple(
                    tokens.get(21).unwrap(),
                    &TokenType::String(
                        Range::new(Position::new(0, 77, 77), Position::new(0, 82, 82)),
                        "float".to_owned(),
                    ),
                );

                equal_token_tuple(
                    tokens.get(23).unwrap(),
                    &TokenType::Float(
                        Range::new(Position::new(0, 85, 85), Position::new(0, 88, 88)),
                        1.0,
                    ),
                );

                equal_token_single(
                    tokens.get(27).unwrap(),
                    &TokenType::ArrayOpen(Range::new(
                        Position::new(0, 99, 99),
                        Position::new(0, 100, 100),
                    )),
                );

                equal_token_single(
                    tokens.get(28).unwrap(),
                    &TokenType::ArrayClose(Range::new(
                        Position::new(0, 100, 100),
                        Position::new(0, 101, 101),
                    )),
                );

                equal_token_single(
                    tokens.get(29).unwrap(),
                    &TokenType::ObjectClose(Range::new(
                        Position::new(0, 102, 102),
                        Position::new(0, 103, 103),
                    )),
                );
            }
            _ => panic!("Could not tokenize json."),
        }
    }

    fn equal_token_tuple(token1: &TokenType, token2: &TokenType) {
        match (token1, token2) {
            (TokenType::String(r1, v1), TokenType::String(r2, v2)) => {
                equal_range(r1, r2);
                assert_eq!(v1, v2, "String not equal. ('{}' != '{}')", v1, v2);
            }
            (TokenType::Bool(r1, v1), TokenType::Bool(r2, v2)) => {
                equal_range(r1, r2);
                assert_eq!(v1, v2, "Bool not equal. ('{}' != '{}')", v1, v2);
            }
            (TokenType::Float(r1, v1), TokenType::Float(r2, v2)) => {
                equal_range(r1, r2);
                assert!(
                    approx_eq!(f64, *v1, *v2, ulps = 2),
                    "Float not equal. ('{}' != '{}')",
                    v1,
                    v2
                );
            }
            (TokenType::Int(r1, v1), TokenType::Int(r2, v2)) => {
                equal_range(r1, r2);
                assert_eq!(v1, v2, "Int not equal. ('{}' != '{}')", v1, v2);
            }
            _ => panic!("Token token '{:?}' does not match '{:?}'", token1, token2),
        }
    }

    fn equal_token_single(token1: &TokenType, token2: &TokenType) {
        match (token1, token2) {
            (TokenType::ObjectOpen(r1), TokenType::ObjectOpen(r2))
            | (TokenType::ObjectClose(r1), TokenType::ObjectClose(r2))
            | (TokenType::ArrayOpen(r1), TokenType::ArrayOpen(r2))
            | (TokenType::ArrayClose(r1), TokenType::ArrayClose(r2))
            | (TokenType::Null(r1), TokenType::Null(r2))
            | (TokenType::Semicolon(r1), TokenType::Semicolon(r2))
            | (TokenType::Comma(r1), TokenType::Comma(r2)) => equal_range(r1, r2),
            _ => panic!(
                "Token token type '{:?}' does not match '{:?}'",
                token1, token2
            ),
        }
    }

    fn equal_range(range1: &Range, range2: &Range) {
        equal_position(&range1.start, &range2.start);
        equal_position(&range1.end, &range2.end);
    }

    fn equal_position(pos1: &Position, pos2: &Position) {
        assert_eq!(
            pos1.idx, pos2.idx,
            "Position not equal. (Idx should be: '{}' is: '{}')",
            pos1.idx, pos2.idx
        );
        assert_eq!(
            pos1.line, pos2.line,
            "Position not equal. (Line should be: '{}' is: '{}')",
            pos1.line, pos2.line
        );
        assert_eq!(
            pos1.char, pos2.char,
            "Position not equal. (Char should be: '{}' is: '{}')",
            pos1.char, pos2.char
        );
    }
}
