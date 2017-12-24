use std::collections::{HashSet, HashMap};
use std::io::{self};
use std::iter::FromIterator;

#[test]
fn examples_part1() {
    assert_eq!(is_valid_line_part1("aa bb cc dd ee"), true);
    assert_eq!(is_valid_line_part1("aa bb cc dd aa"), false);
    assert_eq!(is_valid_line_part1("aa bb cc dd aaa"), true);
}

#[test]
fn examples_part2() {
    assert_eq!(is_valid_line_part2("abcde fghij"),  true);
    assert_eq!(is_valid_line_part2("abcde xyz ecdab"),  false);
    assert_eq!(is_valid_line_part2("a ab abc abd abf abj"),  true);
    assert_eq!(is_valid_line_part2("iiii oiii ooii oooi oooo"),  true);
    assert_eq!(is_valid_line_part2("oiii ioii iioi iiio"),  false);
}

#[test]
fn is_anagram_tests() {
    assert_eq!(is_anagram("", ""), true);
    assert_eq!(is_anagram("anna", "nana"), true);
    assert_eq!(is_anagram("bob", "obb"), true);
    assert_eq!(is_anagram("qwerty", "asdfg"), false);
    assert_eq!(is_anagram("aa", "a"), false);
    assert_eq!(is_anagram("abb", "aab"), false);
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false; }

    let mut map = HashMap::new();

    for c in s1.chars() {
        let value = match map.get(&c) {
            Some(value) => value + 1,
            None => 1
        };
        map.insert(c, value);
    }

    for c in s2.chars() {
        if map.contains_key(&c) == false { return false; }
        let value = map.get(&c).unwrap() - 1;
        if value < 0 { return false; }
        map.insert(c, value);
    }

    true
}

fn is_valid_line_part1(input: &str) -> bool {
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

fn is_valid_line_part2(input: &str) -> bool {
    if is_valid_line_part1(input) == false { return false; }

    let words = Vec::from_iter(input.split_whitespace().map(String::from));

    for (i, word1) in words.iter().enumerate() {
        for (j, word2) in words.iter().enumerate() {
            if i == j { continue; }
            if is_anagram(word1, word2) { return false; }
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

        if is_valid_line_part2(&line) { number_of_valid_lines += 1; }
    }

    println!("number_of_valid_lines={}", number_of_valid_lines);
}
