use std::collections::HashMap;
use std::cmp::{max, min};

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let threads = parse_input(input);
    let max_num = 32;
    let count = threads.iter().filter(|&(a, b)| b-a == max_num/2).count();
    count.to_string()
}

fn part2(input: &str) -> String {
    let threads = parse_input(input);
    let mut thread_counts: HashMap<(i32,i32),i32> = HashMap::new();
    for thread in threads {
        *thread_counts.entry(thread).or_insert(0) += 1;
    }
    let mut knot_count = 0;
    for (thread1,value) in thread_counts.iter() {
        knot_count += thread_counts.keys()
            .filter(|thread2| are_crossing(thread1, *thread2))
            .fold(0, |acc, thread2| {acc + thread_counts.get(thread2).unwrap()});
    }
    (knot_count/2).to_string()  // divide by two since we're double counting
}


fn part3(input: &str) -> String {
    let threads = parse_input(input);
    let max_value = 255;
    let mut thread_counts: HashMap<(i32,i32),i32> = HashMap::new();
    for thread in threads {
        *thread_counts.entry(thread).or_insert(0) += 1;
    }
    let mut max_knots: Vec<i32> = vec![];
    for a in 1..max_value {
        for b in a+2..max_value {
            //if a == b {continue}
            let cut_threads = thread_counts.iter().fold(0, |acc, (thread2, count)| if are_crossing(&(a,b), thread2) {*count + acc} else {acc});
            max_knots.push(cut_threads);
        }
    }
    max_knots.iter().max().unwrap().to_string()
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let nails: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    nails.windows(2).map(|n| (*n.iter().min().unwrap(), *n.iter().max().unwrap())).collect()
}

fn are_crossing(a: &(i32, i32), b: &(i32, i32)) -> bool {
    if a.0 == b.0 || a.0 == b.1 || a.1 == b.0 || a.1 == b.1 {
        return false;
    }
    if (b.0 > a.0 && b.0 < a.1) != (b.1 > a.0 && b.1 < a.1) {
        return true
    }
    false
}
