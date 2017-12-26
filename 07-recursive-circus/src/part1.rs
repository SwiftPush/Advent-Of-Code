use std::iter::FromIterator;
use std::str::FromStr;
use std::collections::{HashMap};
use std::io::{self, Read};

struct Node {
    name: String,
    weight: i32,
    parents: Vec<String>,
    children: Vec<String>,
}

#[test]
fn example1() {
    let input: Vec<String> = vec![
        String::from("pbga (66)"),
        String::from("xhth (57)"),
        String::from("ebii (61)"),
        String::from("havc (66)"),
        String::from("ktlj (57)"),
        String::from("fwft (72) -> ktlj, cntj, xhth"),
        String::from("qoyq (66)"),
        String::from("padx (45) -> pbga, havc, qoyq"),
        String::from("tknk (41) -> ugml, padx, fwft"),
        String::from("jptl (61)"),
        String::from("ugml (68) -> gyxo, ebii, jptl"),
        String::from("gyxo (61)"),
        String::from("cntj (57)"),
    ];
    assert_eq!(solution1(input), "tknk");
}

fn parse_row(input: &str) -> Node {
    let words: Vec<String> = Vec::from_iter(input.split_whitespace().map(String::from));
    if !(words.len() == 2 || words.len() >= 4) { panic!("Invalid format, {}", input); }
    let mut words_iter = words.iter();

    let name: String = String::from_str(words_iter.next().unwrap()).unwrap();
    let chars_to_trim: &[char] = &['(', ')', ','];
    let weight = words_iter.next().unwrap().trim_matches(chars_to_trim).parse::<i32>().unwrap();

    let _arrow = words_iter.next();

    let parents: Vec<String> = Vec::from_iter(words_iter.map(|s| s.trim_matches(chars_to_trim).to_string()));

    let node = Node {
        name: name,
        weight: weight,
        parents: parents,
        children: vec![],
    };

    node
}

fn populate_children(nodes: &mut HashMap<String, Node>) {
    let mut node_names: Vec<String> = vec![];
    for (key, _node) in nodes.iter() {
        node_names.push(String::from_str(key).unwrap());
    }

    for node in &node_names {
        let parents: Vec<String> = nodes.get(node).unwrap().parents.to_owned();
        for parent in &parents {
            let parent_node = nodes.get_mut(parent).unwrap();
            parent_node.children.push(node.to_string());
        }
    }
}

fn solution1(input: Vec<String>) -> String {
    let mut nodes = HashMap::new();

    for row in &input {
        if row.is_empty() { continue; }
        let node = parse_row(row);
        nodes.insert(node.name.to_string(), node);
    }

    /*for (key, node) in &nodes {
        println!("key={} name={} weight={} parents={} children={}", key, node.name, node.weight, node.parents.len(), node.children.len());
    }*/

    populate_children(&mut nodes);
    /*println!("children populated");

    for (key, node) in &nodes {
        println!("key={} name={} weight={} parents={} children={}", key, node.name, node.weight, node.parents.len(), node.children.len());
    }*/

    let (_key, mut random_node) = nodes.iter().next().unwrap();
    while random_node.children.len() > 0 {
        let random_node_name = random_node.children.iter().next().unwrap();
        random_node = nodes.get(random_node_name).unwrap();
    }

    random_node.name.to_string()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let lines: Vec<String> = Vec::from_iter(buffer.split('\n').map(String::from));
    let solution = solution1(lines);
    println!("{}", solution);
}
