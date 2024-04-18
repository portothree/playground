use std::io::{BufReader, Cursor};

#[test]
fn find_a_match() {
    let input = b"lorem";
    let mut reader = BufReader::new(Cursor::new(input));
    let mut writter = Vec::new();

    let result = grrs::find_matches(&mut reader, "lorem", &mut writter);

    assert_eq!(result, "lorem");
}
