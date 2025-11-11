use std::collections::{HashMap, VecDeque};
use std::iter::repeat_n;

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let mut pair_count = 0;
    let mut mentor_counts = HashMap::new();
    for char in input.chars() {
        if char == 'A' {
            let entry = mentor_counts.entry(char).or_insert(0);
            *entry += 1;
        }
        else if char == 'a' {
            pair_count += *mentor_counts.entry(char.to_ascii_uppercase()).or_insert(0);
        }
    }
    pair_count.to_string()
}

fn part2(input: &str) -> String {
    let mut pair_count = 0;
    let mut mentor_counts = HashMap::new();
    for char in input.chars() {
        if char.is_uppercase() {
            *mentor_counts.entry(char).or_insert(0) += 1;
        }
        else {
            pair_count += *mentor_counts.entry(char.to_ascii_uppercase()).or_insert(0);
        }
    }
    pair_count.to_string()
}

fn part3(input: &str) -> String {
    let input_repeat = repeat_n(input, 3).collect::<String>();
    let buffer_length = 1000;

    let mut pair_count = 0;
    let mut mentor_counts: HashMap<char, i32> = HashMap::new();
    let mut front_buffer:VecDeque<char> = VecDeque::new();
    let mut back_buffer:VecDeque<char> = VecDeque::new();
    for (index, char) in input_repeat.chars().enumerate() {
        if char.is_uppercase() {
            *mentor_counts.entry(char).or_insert(0) += 1;
        }
        front_buffer.push_front(char);
        if index > (buffer_length - 1) as usize {
            let item = front_buffer.pop_back().unwrap();

            if index > (buffer_length * 2) as usize {
                let back_item = back_buffer.pop_back().unwrap();
                if back_item.is_uppercase() {
                    *mentor_counts.entry(back_item).or_insert(0) -= 1;
                }
            }
            if item.is_lowercase() {
                if index >= 11000 && index < 21000 {  // count middle section times 998 instead of repeating identical sections
                    pair_count += *mentor_counts.entry(item.to_ascii_uppercase()).or_insert(0) * 998;
                }
                else {
                    pair_count += *mentor_counts.entry(item.to_ascii_uppercase()).or_insert(0);
                }
            }
            back_buffer.push_front(item);
        }
    }
    // We need to count through the front buffer to not the last {buffer_length} items
    while !front_buffer.is_empty() {
        let item = front_buffer.pop_back().unwrap();

        let back_item = back_buffer.pop_back().unwrap();
        if back_item.is_uppercase() {
            *mentor_counts.entry(back_item).or_insert(0) -= 1;
        }
        if item.is_lowercase() {
            pair_count += *mentor_counts.entry(item.to_ascii_uppercase()).or_insert(0);
        }
        back_buffer.push_front(item);
    }
    pair_count.to_string()
}
