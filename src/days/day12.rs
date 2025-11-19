use std::collections::{HashMap, HashSet};

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let (grid, max_size) = parse_input(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));
    let mut to_visit = vec![(0, 0)];
    while !to_visit.is_empty() {
        let pos = to_visit.pop().unwrap();
        for neighbor in get_neighbors(&pos, &max_size) {
            if grid.get(&pos) >= grid.get(&neighbor) && !visited.contains(&neighbor) {
                to_visit.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    visited.len().to_string()
}

fn part2(input: &str) -> String {
    let (grid, max_size) = parse_input(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));
    let mut to_visit = vec![(0, 0), (max_size.0-1, max_size.1-1)];
    while !to_visit.is_empty() {
        let pos = to_visit.pop().unwrap();
        for neighbor in get_neighbors(&pos, &max_size) {
            if grid.get(&pos) >= grid.get(&neighbor) && !visited.contains(&neighbor) {
                to_visit.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    visited.len().to_string()
}

fn part3(input: &str) -> String {
    let (grid, max_size) = parse_input(input);
    let mut all_burnt = HashSet::new();
    let mut targets = grid.keys()
        .filter(|pos| is_peak(pos, &grid, &max_size))
        .cloned().collect::<Vec<_>>();
    targets.sort_by_key(|&pos| grid.get(&pos).unwrap());
    targets.reverse();

    for _ in 0..3 {
        let mut most_burnt = HashSet::new();
        for key in targets.iter() {
            if all_burnt.contains(key) {continue}
            let latest_throw = throw_fireball(*key, &grid, &max_size, &all_burnt);

            if latest_throw.len() > most_burnt.len() {
                most_burnt = latest_throw;
            }
        }
        all_burnt.extend(&most_burnt);
    }
    all_burnt.len().to_string()
}

fn parse_input(input: &str) -> (HashMap<(usize, usize), u32>, (usize, usize)) {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.insert((x, y), char::to_digit(char, 10).unwrap());
        }
    }
    (grid, (input.lines().into_iter().nth(0).unwrap().len(),input.lines().count()))
}

fn get_neighbors(pos: &(usize, usize), max_size: &(usize, usize)) -> Vec<(usize, usize)> {
    let test = vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1), (pos.0, pos.1 - 1)].into_iter()
        .filter(|pos| pos.0 < max_size.0 && pos.1 < max_size.1)
        .collect();
    test
}

fn throw_fireball(start: (usize, usize), grid: &HashMap<(usize, usize), u32>, max_size: &(usize, usize), all_burnt: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut to_visit = vec![start];
    while let Some(pos) = to_visit.pop() {
        for neighbor in get_neighbors(&pos, max_size) {
            if grid.get(&pos) >= grid.get(&neighbor) && !visited.contains(&neighbor) && !all_burnt.contains(&neighbor) {
                to_visit.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    visited
}

fn is_peak(pos: &(usize, usize), grid: &HashMap<(usize, usize), u32>, max_size: &(usize, usize)) -> bool {
    let value = grid.get(&pos);
    get_neighbors(pos, max_size).iter().all(|neighbor| value > grid.get(neighbor))
}