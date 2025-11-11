use std::collections::{HashMap, HashSet};

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let unique: HashSet<_> = HashSet::from_iter(parse_input(input));
    unique.iter().sum::<i32>().to_string()
}

fn part2(input: &str) -> String {
    let unique: HashSet<_> = HashSet::from_iter(parse_input(input));
    let mut vec: Vec<_> = unique.into_iter().collect();
    vec.sort();
    vec[0..20].iter().sum::<i32>().to_string()
}

fn part3(input: &str) -> String {
    let mut counts = HashMap::new();
    parse_input(input).iter().for_each(|x| *counts.entry(*x).or_insert(0) += 1);
    counts.values().max().unwrap().to_string()
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.parse::<_>().unwrap()).collect()
}