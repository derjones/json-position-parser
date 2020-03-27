use super::types::Range;
use std::collections::HashMap;

#[derive(Debug)]
pub enum EntryType {
    JSONObject(HashMap<String, (usize, usize)>), // key, value
    JSONArray(Vec<usize>),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub struct Entry {
    pub key: Option<usize>,
    pub range: Range,
    pub entry_type: EntryType,
}

#[derive(Debug)]
pub struct Key {
    pub name: String,
    pub range: Range,
}

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<Entry>,
    pub keys: Vec<Key>,
}

#[derive(Debug)]
pub enum PathType<'input> {
    Object(&'input str),
    Array(usize),
    Wildcard,
    RecursiveWildcard,
}

impl Tree {
    fn handle_path(&self, entries: &[&Entry], path: &str) -> Vec<&Entry> {
        entries
            .iter()
            .filter_map(|entry| {
                if let EntryType::JSONObject(entries) = &entry.entry_type {
                    if let Some((_, value)) = entries.get(path) {
                        return self.entries.get(*value);
                    }
                }
                None
            })
            .collect()
    }

    fn handle_array(&self, entries: &[&Entry], pos: usize) -> Vec<&Entry> {
        entries
            .iter()
            .filter_map(|entry| {
                if let EntryType::JSONArray(entries) = &entry.entry_type {
                    if let Some(value) = entries.get(pos) {
                        return self.entries.get(*value);
                    }
                }
                None
            })
            .collect()
    }

    fn handle_wildcard(&self, entries: &[&Entry]) -> Vec<&Entry> {
        entries
            .iter()
            .filter_map(|entry| {
                if let EntryType::JSONObject(hash) = &entry.entry_type {
                    return Some(
                        hash.iter()
                            .filter_map(|(_, (_, entry))| self.entries.get(*entry))
                            .collect(),
                    );
                }

                if let EntryType::JSONArray(array) = &entry.entry_type {
                    return Some(
                        array
                            .iter()
                            .filter_map(|entry| self.entries.get(*entry))
                            .collect(),
                    );
                }

                None
            })
            .collect::<Vec<Vec<&Entry>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&Entry>>()
    }

    fn handle_recursive_wildcard(&self, entries: &[&Entry]) -> Vec<&Entry> {
        entries
            .iter()
            .filter_map(|entry| {
                if let EntryType::JSONObject(hash) = &entry.entry_type {
                    let values: Vec<&Entry> = hash
                        .iter()
                        .filter_map(|(_, (_, entry))| self.entries.get(*entry))
                        .collect();
                    let next = self.handle_recursive_wildcard(&values);
                    return Some([&values[..], &next[..]].concat());
                }

                if let EntryType::JSONArray(array) = &entry.entry_type {
                    let values: Vec<&Entry> = array
                        .iter()
                        .filter_map(|entry| self.entries.get(*entry))
                        .collect();
                    let next = self.handle_recursive_wildcard(&values);
                    return Some([&values[..], &next[..]].concat());
                }

                None
            })
            .collect::<Vec<Vec<&Entry>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&Entry>>()
    }

    /// Get values at a path. Wildcard and recursive wildcard available.
    ///
    /// **Example**
    /// ```
    /// let text = "{ \"a\": { \"b\": ["c"] } }";
    /// match parse_json(&text) {
    ///     Ok(tree) => {
    ///         let keys = tree.values_at([
    ///            PathType::Object("a"),
    ///            PathType::Object("b"),
    ///            PathType::Array(0),
    ///         ]);
    ///     },
    ///     Err(e) => println!("{:?}", e),
    /// };
    /// ```
    pub fn value_at(&self, path: &[PathType]) -> Vec<&Entry> {
        let mut vec = vec![];
        if let Some(first) = self.entries.last() {
            vec.push(first);
            return path.iter().fold(vec, |last, path| match path {
                PathType::Object(path) => self.handle_path(&last, path),
                PathType::Array(pos) => self.handle_array(&last, *pos),
                PathType::Wildcard => self.handle_wildcard(&last),
                PathType::RecursiveWildcard => {
                    [&last[..], &self.handle_recursive_wildcard(&last)[..]].concat()
                }
            });
        }

        vec
    }

    /// Get keys at a path. Wildcard and recursive wildcard available.
    ///
    /// **Example**
    /// ```
    /// let text = "{ \"a\": { \"b\": ["c"] } }";
    /// match parse_json(&text) {
    ///     Ok(tree) => {
    ///         let keys = tree.keys_at([
    ///            PathType::Object("a"),
    ///            PathType::Object("b"),
    ///            PathType::Array(0),
    ///         ]);
    ///     },
    ///     Err(e) => println!("{:?}", e),
    /// };
    /// ```
    pub fn keys_at(&self, path: &[PathType]) -> Vec<&Key> {
        self.value_at(path)
            .iter()
            .filter_map(|entry| {
                if let EntryType::JSONObject(hash) = &entry.entry_type {
                    return Some(
                        hash.iter()
                            .filter_map(|(_, (key, _))| self.keys.get(*key))
                            .collect(),
                    );
                }

                None
            })
            .collect::<Vec<Vec<&Key>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&Key>>()
    }
}
