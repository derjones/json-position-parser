use super::types::{Position, Range};

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
    Comma,
    Semicolon,
}

fn handle_defaults(c: char, tokens: &mut Vec<TokenType>, pos: Position) {
    match c {
        '{' => {
            tokens.push(TokenType::ObjectOpen(Range {
                start: pos,
                end: pos.add(0, 1),
            }));
        }
        '}' => {
            tokens.push(TokenType::ObjectClose(Range {
                start: pos,
                end: pos.add(0, 1),
            }));
        }
        '[' => {
            tokens.push(TokenType::ArrayOpen(Range {
                start: pos,
                end: pos.add(0, 1),
            }));
        }
        ']' => {
            tokens.push(TokenType::ArrayClose(Range {
                start: pos,
                end: pos.add(0, 1),
            }));
        }
        ',' => {
            tokens.push(TokenType::Comma);
        }
        ':' => {
            tokens.push(TokenType::Semicolon);
        }
        _ => {}
    }
}

pub fn tokenize(string: &str, tokens: &mut Vec<TokenType>) {
    let mut current_type = None;
    let mut current_type_start = Position::default();
    let mut concat_string = String::new();
    let mut current_line = 0;
    let mut current_char = 0;
    let mut escaped = false;

    string.chars().enumerate().for_each(|(pos, c)| {
        if let '\n' = c {
            current_line += 1;
            current_char = 0;
        } else {
            match &current_type {
                Some(t) => {
                    match t {
                        CurrentTokenType::String => {
                            if !escaped && c == '\\' {
                                escaped = true;
                                concat_string.push(c);
                            } else if !escaped && c == '"' {
                                tokens.push(TokenType::String(
                                    Range {
                                        start: current_type_start.add(0, 1),
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
                                }
                                current_type = None;
                                concat_string = String::new();
                                handle_defaults(
                                    c,
                                    tokens,
                                    Position::new(current_line, current_char, pos),
                                );
                            } else {
                                concat_string.push(c);
                            }
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
                                    tokens,
                                    Position::new(current_line, current_char, pos),
                                );
                            } else {
                                concat_string.push(c);
                            }
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
                                    tokens,
                                    Position::new(current_line, current_char, pos),
                                );
                            } else {
                                concat_string.push(c);
                            }
                        }
                    };
                }
                None => match c {
                    '"' => {
                        current_type = Some(CurrentTokenType::String);
                        current_type_start = Position::new(current_line, current_char, pos);
                    }
                    'n' => {
                        current_type = Some(CurrentTokenType::Null);
                        current_type_start = Position::new(current_line, current_char, pos);
                        concat_string.push(c);
                    }
                    't' | 'f' => {
                        current_type = Some(CurrentTokenType::Bool);
                        current_type_start = Position::new(current_line, current_char, pos);
                        concat_string.push(c);
                    }
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        current_type = Some(CurrentTokenType::Number);
                        current_type_start = Position::new(current_line, current_char, pos);
                        concat_string.push(c);
                    }
                    _ => handle_defaults(c, tokens, Position::new(current_line, current_char, pos)),
                },
            };
            current_char += 1;
        }
    });
}
