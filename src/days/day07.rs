use std::collections::{HashMap, HashSet};

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let (names, rules) = parse_input(input);
    for name in names {
        if validate_name(&name, &rules) {
            return name.to_string();
        }
    }
    "Name not found".to_string()
}

fn part2(input: &str) -> String {
    let (names, rules) = parse_input(input);
    let mut possible_names = 0;
    for (index, name) in names.into_iter().enumerate() {
        if validate_name(&name, &rules) {
            possible_names += index + 1;
        }
    }
    possible_names.to_string()
}

fn part3(input: &str) -> String {
    let (names, rules) = parse_input(input);
    let mut possible_names: HashSet<String> = HashSet::new();
    for name in names {
        if !validate_name(&name, &rules) || possible_names.contains(name) {continue}  // Skip invalid and duplicate names
        let mut to_process = vec![name.to_string()];
        while !to_process.is_empty() {
            let new_names: Vec<String> = to_process.into_iter()
                .flat_map(|name| {
                generate_names(&name, &rules)
            }).collect();
            if new_names[0].len() >= 7 {
                possible_names.extend(new_names.clone());
            }
            if new_names[0].len() <= 10 {
                to_process = new_names;
            }
            else {
                to_process = vec![];
            }
        }
    }
    possible_names.len().to_string()
}

fn parse_input(input: &str) -> (Vec<&str>, HashMap<char, Vec<char>>) {
    let names = input.lines().nth(0).unwrap().split(',').collect();

    let rules: HashMap <char, Vec<char>> = input.lines().skip(2).map(|line| {
        let (letter, results) = line.split_once(" > ").unwrap();
        (letter.chars().nth(0).unwrap(), results.replace(',',"").chars().collect())
    }).collect();

    (names, rules)
}

fn validate_name(name: &str, rules: &HashMap<char, Vec<char>>) -> bool {
    for (a, b) in name.chars().zip(name.chars().skip(1)) { // pairwise iter
        if !rules[&a].iter().any(|c| *c == b) {
            return false
        }
    }
    true
}

fn generate_names(name: &str, rules: &HashMap<char, Vec<char>>) -> Vec<String> {
    match rules.get(&name.chars().last().unwrap()) {
        Some(chars) => {chars.iter()
            .map( | c | format!("{name}{c}")).collect()}
        None => { // if no valid rule
            Vec::new()
        }
    }
}
