# json-position-parser
A simple json parser with positions.


Example:
```
let json = "{ \"a\": {}, \"b\": { \"c\": [true, { \"e\": 42 } ] }, \"f\": [false, { \"e\": 21 } } ] } }";
match super::parse_json(json) {
    Ok(tree) => {
        //
        // Get value at path
        //
        let res = tree.value_at(&[PathType::Object("a")]);
        // [Entry { key: Some(0), range: Range { start: Position { line: 0, char: 7, idx: 7 }, end: Position { line: 0, char: 9, idx: 9 } }, entry_type: JSONObject({}) }]
        println!("{:?}", res);

        //
        // Get value at path with array position
        //
        let res = tree.value_at(&[
            PathType::Object("b"),
            PathType::Object("c"),
            PathType::Array(1),
            PathType::Object("e"),
        ]);
        // [Entry { key: Some(3), range: Range { start: Position { line: 0, char: 37, idx: 37 }, end: Position { line: 0, char: 39, idx: 39 } }, entry_type: Int(42) }]
        println!("{:?}", res);

        //
        // Get value at path with wildcard
        //
        let res = tree.value_at(&[
            PathType::Wildcard,
            PathType::Object("c"),
            PathType::Array(0),
        ]);
        // [Entry { key: None, range: Range { start: Position { line: 0, char: 24, idx: 24 }, end: Position { line: 0, char: 28, idx: 28 } }, entry_type: Bool(true) }]
        println!("{:?}", res);

        //
        // Get value at path with recursive wildcard
        //
        let res = tree.value_at(&[
            PathType::RecursiveWildcard,
            PathType::Array(1),
            PathType::Object("e"),
        ]);

        // [Entry { key: Some(5), range: Range { start: Position { line: 0, char: 67, idx: 67 }, end: Position { line: 0, char: 69, idx: 69 } }, entry_type: Int(21) }, Entry { key: Some(3), range: Range { start: Position { line: 0, char: 37, idx: 37 }, end: Position { line: 0, char: 39, idx: 39 } }, entry_type: Int(42) }]
        println!("{:?}", res);
    }
    _ => panic!("Could not parse json."),
}
```