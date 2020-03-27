use super::types::{Position, Range, ParseError, ParseResult};
use super::tokenize::TokenType;
use super::tree::{Entry, EntryType, Key, Tree};
use std::collections::HashMap;

fn pre_calculate_positions(tokens: &[TokenType]) -> HashMap<usize, usize> {
    let mut hash = HashMap::new();
    let mut array_stack = vec![];
    let mut object_stack = vec![];

    tokens
        .iter()
        .enumerate()
        .for_each(|(idx, token)| match token {
            TokenType::ObjectOpen(_) => object_stack.push(idx),
            TokenType::ObjectClose(_) => {
                if let Some(start) = object_stack.pop() {
                    hash.insert(start, idx);
                }
            }
            TokenType::ArrayOpen(_) => array_stack.push(idx),
            TokenType::ArrayClose(_) => {
                if let Some(start) = array_stack.pop() {
                    hash.insert(start, idx);
                }
            }
            _ => {}
        });

    hash
}

fn create_new_object(tokens: &[TokenType]) -> (HashMap<String, (usize, usize)>, Range) {
    let json_object = HashMap::new();

    let pos_start = if let Some(token) = tokens.first() {
        match token {
            TokenType::ObjectOpen(range) => range.start,
            _ => Position::default(),
        }
    } else {
        Position::default()
    };

    let pos_end = if let Some(token) = tokens.last() {
        match token {
            TokenType::ObjectClose(range) => range.end,
            _ => Position::default(),
        }
    } else {
        Position::default()
    };

    let range = Range {
        start: pos_start,
        end: pos_end,
    };

    (json_object, range)
}

fn create_new_array(tokens: &[TokenType]) -> (Vec<usize>, Range) {
    let json_array = vec![];

    let pos_start = if let Some(token) = tokens.first() {
        match token {
            TokenType::ArrayOpen(range) => range.start,
            _ => Position::default(),
        }
    } else {
        println!("Could not find start pos for array");
        Position::default()
    };

    let pos_end = if let Some(token) = tokens.last() {
        match token {
            TokenType::ArrayClose(range) => range.end,
            _ => Position::default(),
        }
    } else {
        println!("Could not find end pos for array");
        Position::default()
    };

    let range = Range {
        start: pos_start,
        end: pos_end,
    };

    (json_array, range)
}

fn handle_primitives(token: &TokenType, key: Option<usize>) -> Option<Entry> {
    match token {
        TokenType::String(range, val) => Some(Entry {
            key,
            entry_type: EntryType::String(val.clone()),
            range: *range,
        }),
        TokenType::Float(range, val) => Some(Entry {
            key,
            entry_type: EntryType::Float(*val),
            range: *range,
        }),
        TokenType::Int(range, val) => Some(Entry {
            key,
            entry_type: EntryType::Int(*val),
            range: *range,
        }),
        TokenType::Bool(range, val) => Some(Entry {
            key,
            entry_type: EntryType::Bool(*val),
            range: *range,
        }),
        TokenType::Null(range) => Some(Entry {
            key,
            entry_type: EntryType::Null,
            range: *range,
        }),
        _ => None,
    }
}

type ArrayParseResult = ParseResult<(Vec<usize>, Range)>;

fn handle_array(
    tree: &mut Tree,
    tokens: &[TokenType],
    pre_calc: &HashMap<usize, usize>,
    pre_pos: usize,
) -> ArrayParseResult {
    let (mut json_array, range) = create_new_array(&tokens);
    let mut skip = 0;

    let result: Result<Vec<()>, ParseError> = tokens[1..(tokens.len() - 1)]
        .iter()
        .enumerate()
        .map(|(idx, token)| {
            if skip > 0 {
                skip -= 1;
                return Ok(());
            }
            match token {
                TokenType::ObjectOpen(_) => {
                    let next_pos = idx + pre_pos + 1;
                    if let Some(close_idx) = pre_calc.get(&next_pos) {
                        if let Ok((hash, range)) = handle_object(
                            tree,
                            &tokens[idx + 1..=*close_idx - pre_pos],
                            pre_calc,
                            next_pos,
                        ) {
                            skip = *close_idx - next_pos;
                            tree.entries.push(Entry {
                                key: None,
                                range,
                                entry_type: EntryType::JSONObject(hash),
                            });
                            json_array.push(tree.entries.len() - 1);
                            return Ok(());
                        }
                    }
                    Err(ParseError::MissingObjectBrace)
                }
                TokenType::ArrayOpen(_) => {
                    let next_pos = idx + pre_pos + 1;
                    if let Some(close_idx) = pre_calc.get(&next_pos) {
                        if let Ok((array_vec, range)) = handle_array(
                            tree,
                            &tokens[idx + 1..=*close_idx - pre_pos],
                            pre_calc,
                            next_pos,
                        ) {
                            skip = *close_idx - next_pos;
                            tree.entries.push(Entry {
                                key: None,
                                range,
                                entry_type: EntryType::JSONArray(array_vec),
                            });
                            json_array.push(tree.entries.len() - 1);
                            return Ok(());
                        }
                    }
                    Err(ParseError::MissingArrayBrace)
                }
                _ => {
                    if let Some(primitive) = handle_primitives(&token, None) {
                        tree.entries.push(primitive);
                        json_array.push(tree.entries.len() - 1);
                    }

                    Ok(())
                }
            }
        })
        .collect();

    match result {
        Ok(_) => Ok((json_array, range)),
        Err(e) => Err(e),
    }
}

type ObjectParseResult = ParseResult<(HashMap<String, (usize, usize)>, Range)>;

fn handle_object(
    tree: &mut Tree,
    tokens: &[TokenType],
    pre_calc: &HashMap<usize, usize>,
    pre_pos: usize,
) -> ObjectParseResult {
    let mut is_key = true;
    let mut key_pos: usize = 0;
    let (mut json_object, range) = create_new_object(&tokens);
    let mut skip = 0;

    let result: Result<Vec<()>, ParseError> = tokens[1..(tokens.len() - 1)]
        .iter()
        .enumerate()
        .map(|(idx, token)| {
            if skip > 0 {
                skip -= 1;
                return Ok(());
            }
            match token {
                TokenType::Comma => {
                    is_key = true;
                    Ok(())
                }
                TokenType::Semicolon => {
                    is_key = false;
                    Ok(())
                }
                TokenType::String(range, key) => {
                    if is_key {
                        tree.keys.push(Key {
                            name: key.clone(),
                            range: *range,
                        });
                        key_pos = tree.keys.len() - 1;
                        return Ok(());
                    }
                    if let Some(primitive) = handle_primitives(&token, Some(key_pos)) {
                        if let Some(key) = tree.keys.get(key_pos) {
                            tree.entries.push(primitive);
                            json_object.insert(key.name.clone(), (key_pos, tree.entries.len() - 1));
                            return Ok(());
                        }
                    }
                    Err(ParseError::Error)
                }
                TokenType::ObjectOpen(_) => {
                    let next_pos = idx + pre_pos + 1;
                    if let Some(close_idx) = pre_calc.get(&next_pos) {
                        if let Ok((hash, range)) = handle_object(
                            tree,
                            &tokens[idx + 1..=*close_idx - pre_pos],
                            pre_calc,
                            next_pos,
                        ) {
                            skip = *close_idx - next_pos;
                            if let Some(key) = tree.keys.get(key_pos) {
                                tree.entries.push(Entry {
                                    key: Some(key_pos),
                                    range,
                                    entry_type: EntryType::JSONObject(hash),
                                });
                                json_object
                                    .insert(key.name.clone(), (key_pos, tree.entries.len() - 1));

                                is_key = true;
                                return Ok(());
                            }
                        }
                    }

                    Err(ParseError::MissingObjectBrace)
                }
                TokenType::ArrayOpen(_) => {
                    let next_pos = idx + pre_pos + 1;
                    if let Some(close_idx) = pre_calc.get(&next_pos) {
                        if let Ok((array_vec, range)) = handle_array(
                            tree,
                            &tokens[idx + 1..=*close_idx - pre_pos],
                            pre_calc,
                            next_pos,
                        ) {
                            skip = *close_idx - next_pos;
                            if let Some(key) = tree.keys.get(key_pos) {
                                tree.entries.push(Entry {
                                    key: Some(key_pos),
                                    range,
                                    entry_type: EntryType::JSONArray(array_vec),
                                });
                                json_object
                                    .insert(key.name.clone(), (key_pos, tree.entries.len() - 1));
                                return Ok(());
                            }
                        }
                    }

                    Err(ParseError::MissingArrayBrace)
                }
                _ => {
                    if let Some(primitive) = handle_primitives(&token, Some(key_pos)) {
                        if let Some(key) = tree.keys.get(key_pos) {
                            tree.entries.push(primitive);
                            json_object.insert(key.name.clone(), (key_pos, tree.entries.len() - 1));
                        }
                    }

                    Ok(())
                }
            }
        })
        .collect();

    match result {
        Ok(_) => Ok((json_object, range)),
        Err(e) => Err(e),
    }
}

pub fn parse_json(tokens: &[TokenType]) -> ParseResult<Tree> {
    let mut tree = Tree {
        entries: vec![],
        keys: vec![],
    };

    let pre_calc = pre_calculate_positions(tokens);
    if pre_calc.get(&0).is_none() {
        return Err(ParseError::MissingObjectBrace);
    }
    match handle_object(&mut tree, tokens, &pre_calc, 0) {
        Ok((hash, range)) => {
            tree.entries.push(Entry {
                key: None,
                range,
                entry_type: EntryType::JSONObject(hash),
            });
            Ok(tree)
        }
        Err(e) => Err(e),
    }
}
