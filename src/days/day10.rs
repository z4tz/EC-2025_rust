use std::collections::{HashMap, HashSet};
use num::complex::Complex;

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let max_pos = input.lines().count() as i64;
    let (sheeps, _, _) = parse_input(input);
    let mut visited: HashSet<Complex<i64>> = HashSet::new();
    visited.insert(Complex::new(max_pos/2, max_pos/2));
    let mut to_visit = HashSet::new();
    to_visit.insert(Complex::new(max_pos/2, max_pos/2));
    for _ in 0..4 {
        let new_locations = to_visit.into_iter().flat_map(|position| get_jump_positions(position, (max_pos, max_pos))).collect::<HashSet<_>>();
        visited.extend(&new_locations);
        to_visit = new_locations;
    }
    visited.intersection(&sheeps).count().to_string()
}

fn part2(input: &str) -> String {
    let max_pos = input.lines().count() as i64;
    let (mut sheeps, hideouts, _) = parse_input(input);
    let mut dragon_positions = HashSet::new();
    dragon_positions.insert(Complex::new(max_pos/2, max_pos/2));
    let mut eaten_count = 0;
    for _ in 0..20 {
        //move dragons
        dragon_positions = dragon_positions.into_iter().flat_map(|position| get_jump_positions(position, (max_pos, max_pos))).collect::<HashSet<_>>();

        //eat sheep
        let eaten = devoured_sheeps(&sheeps, &hideouts, &dragon_positions);
        eaten_count += eaten.len();
        sheeps = sheeps.difference(&eaten).cloned().collect::<HashSet<_>>();
        // move sheep
        sheeps = move_sheep(sheeps, max_pos);
        //eat sheep again
        let eaten = devoured_sheeps(&sheeps, &hideouts, &dragon_positions);
        eaten_count += eaten.len();
        sheeps = sheeps.difference(&eaten).cloned().collect::<HashSet<_>>();
    }
    eaten_count.to_string()
}

fn part3(input: &str) -> String {
    let (sheeps, hideouts, dragon_position) = parse_input(input);
    let max_pos = (input.lines().nth(0).unwrap().trim().len() as i64,input.lines().count() as i64);
    let mut cache: HashMap<(Complex<i64>, Vec<Complex<i64>>), usize> = HashMap::new();
    let mut dragon_move_cache: HashMap<Complex<i64>, Vec<Complex<i64>>> = HashMap::new();
    for y in 0..max_pos.1 {
        for x in 0..max_pos.0 {
            dragon_move_cache.insert(Complex::new(x, y), get_jump_positions(Complex::new(x, y), max_pos));
        }
    }

    let sequence_count = recursive_turn(dragon_position, sheeps.into_iter().collect(), &hideouts, max_pos, &mut cache, &dragon_move_cache);

    sequence_count.to_string()
}

fn parse_input(input: &str) -> (HashSet<Complex<i64>>, HashSet<Complex<i64>>, Complex<i64>) {
    let mut sheeps = HashSet::new();
    let mut hideouts = HashSet::new();
    let mut dragon_positions = Complex::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                'S' => {sheeps.insert(Complex::new(x as i64, y as i64));},
                '#' => {hideouts.insert(Complex::new(x as i64, y as i64));},
                'D' => {dragon_positions = Complex::new(x as i64, y as i64);},
                _ => {}
            }
        }
    }
    (sheeps, hideouts, dragon_positions)
}

fn get_jump_positions(position: Complex<i64>, max_pos: (i64, i64)) -> Vec<Complex<i64>> {
    let mut positions = vec![];
    for i in  0..4 {
        positions.push(position + (Complex::new(2, 1) * Complex::i().powi(i)));
        positions.push(position + (Complex::new(2, -1) * Complex::i().powi(i)));
    }
    positions.into_iter().filter(|pos| {
        pos.re >= 0 && pos.im >= 0 && pos.re < max_pos.0 && pos.im < max_pos.1
    }).collect::<Vec<Complex<i64>>>()
}


fn move_sheep(sheeps: HashSet<Complex<i64>>, max_pos: i64) -> HashSet<Complex<i64>> {
    sheeps.iter()
        .map(|sheep| Complex::new(sheep.re, sheep.im+1))
        .filter(|c| c.im < max_pos)
        .collect()
}

fn devoured_sheeps(sheeps: &HashSet<Complex<i64>>, hideouts: &HashSet<Complex<i64>>, dragon_positions: &HashSet<Complex<i64>>) -> HashSet<Complex<i64>> {
    sheeps.iter().filter(|sheep| !hideouts.contains(sheep) && dragon_positions.contains(sheep)).cloned().collect::<HashSet<_>>()
}


fn recursive_turn(dragon: Complex<i64>, mut sheeps: Vec<Complex<i64>>, hideouts: &HashSet<Complex<i64>>, max_pos: (i64, i64), cache: &mut HashMap<(Complex<i64>, Vec<Complex<i64>>), usize>, dragon_move_cache: &HashMap<Complex<i64>, Vec<Complex<i64>>>) -> usize {
    if sheeps.iter().any(|sheep| sheep.im >= max_pos.1) {return 0}

    if sheeps.contains(&dragon) && !hideouts.contains(&dragon) {
        if sheeps.len() == 1 {return 1}
        else {  // if more sheep left remove and continue
            sheeps.remove(sheeps.iter().position(|sheep| sheep == &dragon).unwrap());
        }
    }
    if cache.contains_key(&(dragon, sheeps.clone())) {
        return cache[&(dragon, sheeps)];
    }

    let mut score = 0;
        let mut move_possible = false;
        for (index, sheep) in sheeps.iter().enumerate() {
            let possible_move = sheep + Complex::new(0, 1);
            if dragon != possible_move ||hideouts.contains(&possible_move)  {
                move_possible = true;
                let mut sheeps_clone = sheeps.clone();
                sheeps_clone[index] = possible_move;
                for dragon_move in dragon_move_cache.get(&dragon).unwrap() {
                    score += recursive_turn(*dragon_move, sheeps_clone.clone(), hideouts, max_pos, cache, dragon_move_cache);
                }
            }
        }
        if !move_possible {
            for dragon_move in dragon_move_cache.get(&dragon).unwrap() {
                score += recursive_turn(*dragon_move, sheeps.clone(), hideouts, max_pos, cache, dragon_move_cache);
            }
        }
    cache.insert((dragon, sheeps), score);
    score
}

// fn print_positions(positions: &HashSet<Complex<i64>>, max_pos: i64) {
//     let mut lines = vec![];
//     for y in 0..max_pos {
//         let mut line = vec![];
//         for x in 0..max_pos {
//             line.push(match positions.contains(&Complex::new(x as i64, y as i64)) {
//                 true => "X",
//                 false => ".",
//             })
//         }
//         lines.push(line.join(""));
//     }
//     println!("{}", lines.join("\n"));
// }
