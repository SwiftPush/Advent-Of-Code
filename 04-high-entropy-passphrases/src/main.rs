use std::collections::HashSet;
use std::io::{self};

#[test]
fn examples() {
    assert_eq!(is_valid("aa bb cc dd ee"), true);
    assert_eq!(is_valid("aa bb cc dd aa"), false);
    assert_eq!(is_valid("aa bb cc dd aaa"), true);
}

fn is_valid(input: &str) -> bool {
    let mut words = HashSet::new();
    let iter = input.split_whitespace();
    for word in iter {
        if words.contains(word) {
            return false;
        } else {
            words.insert(word);
        }
    }

    true
}

fn main() {
    let mut number_of_valid_lines: usize = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() { break; }

        if is_valid(&line) { number_of_valid_lines += 1; }
    }

    println!("number_of_valid_lines={}", number_of_valid_lines);
}
