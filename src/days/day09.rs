use std::collections::{HashMap, HashSet};

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let sequences = parse_input(input);
    let mut values = vec![];
    for i in 0..sequences.len() {
        for j in i..sequences.len() {
            if i == j {continue;}
            values.push(get_matches(&sequences[i].1, &sequences[j].1));
        }
    }
    values.sort();
    values.reverse();
    (values[0] * values[1]).to_string()
}

fn part2(input: &str) -> String {
    let sequences = parse_input(input);
    let mut total_similarity = 0;
    for i in 0..sequences.len() {
        for j in 0..sequences.len() {
            for k in j..sequences.len() {
                if i == j || i==k || j==k {continue;}
                if are_parents(&sequences[i].1, &sequences[j].1, &sequences[k].1) {
                    total_similarity += get_matches(&sequences[i].1, &sequences[j].1) * get_matches(&sequences[i].1, &sequences[k].1)
                }
            }
        }
    }
    total_similarity.to_string()
}

fn part3(input: &str) -> String {
    let sequences = parse_input(input);
    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..sequences.len() {
        for j in 0..sequences.len() {
            for k in j..sequences.len() {
                if i == j || i==k || j==k {continue;}
                if are_parents(&sequences[i].1, &sequences[j].1, &sequences[k].1) {
                    connections.entry(sequences[i].0).or_insert(vec![]).extend(vec![sequences[j].0, sequences[k].0]);
                    connections.entry(sequences[j].0).or_insert(vec![]).extend(vec![sequences[i].0, sequences[k].0]);
                    connections.entry(sequences[k].0).or_insert(vec![]).extend(vec![sequences[i].0, sequences[j].0]);
                }
            }
        }
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut family_sizes: Vec<(usize, usize)> = vec![];
    for i in connections.keys() {
        if visited.contains(i) {continue;}
        let related = gather_related(*i, &connections);
        visited.extend(&related);
        family_sizes.push((related.len(),related.iter().sum()));
    }

    family_sizes.iter().max().unwrap().1.to_string()
}

fn parse_input(input: &str) -> Vec<(usize, Vec<char>)> {
    let mut items = Vec::new();
    for line in input.lines() {
        let parts = line.split_once(':').unwrap();
        items.push((parts.0.parse().unwrap(), parts.1.chars().collect()));
    }
    items
}

fn get_matches(s1: &Vec<char>, s2: &Vec<char>) -> usize {
    s1.iter().zip(s2.iter()).filter(|&(a, b)| a == b).count()
}

fn are_parents(child: &Vec<char>, parent1: &Vec<char>, parent2: &Vec<char>) -> bool {
    for (c, (p1, p2)) in child.iter().zip(parent1.iter().zip(parent2.iter())) {
        if c != p1 && c != p2 {
            return false;
        }
    }
    true
}

fn gather_related(start: usize, connections: &HashMap<usize, Vec<usize>>) -> HashSet<usize> {
    let mut related = HashSet::new();
    let mut to_process = vec![start];
    while !to_process.is_empty() {
        let new_related: Vec<usize> = connections[&to_process.pop().unwrap()].iter()
            .filter(|&c| !related.contains(c) && !to_process.contains(&c))
            .map(|c| *c)
            .collect();
        related.extend(new_related.clone());
        to_process.extend(new_related.iter().map(|&c| c).collect::<Vec<usize>>());
    }
    related
}