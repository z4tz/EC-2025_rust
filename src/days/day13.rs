use std::collections::VecDeque;

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let  numbers = parse_input(input);
    let mut wheel = VecDeque::new();
    wheel.push_front(1);
    for (i, num) in numbers.into_iter().enumerate() {
        match i%2==0 {
            true => {wheel.push_back(num)},
            false => {wheel.push_front(num)},
        }
    }
    let start_pos = wheel.iter().position(|&x| x == 1).unwrap();
    wheel.rotate_left(2025%wheel.len());
    wheel[start_pos].to_string()
}

fn part2(input: &str) -> String {
    let numbers = parse_input_ranges(input);
    solve(numbers, 20252025)
}

fn part3(input: &str) -> String {
    let numbers = parse_input_ranges(input);
    solve(numbers, 202520252025)
}

fn solve(numbers: Vec<[usize;2]>, rotations: usize) -> String {
    let mut wheel_length = 1;  // to account for the 1 already in the wheel
    let mut range_wheel = VecDeque::new();
    range_wheel.push_front(([1,1], false));
    for (i, nums) in numbers.into_iter().enumerate() {
        match i%2==0 {
            true => {range_wheel.push_back((nums, false))},
            false => {range_wheel.push_front((nums, true))},  // true for indicating that range should be treated in reverse
        }
        wheel_length += nums[1] - nums[0] + 1;
    }

    let final_position = rotations%wheel_length;
    let start_pos = range_wheel.iter().position(|&x| x.0 == [1,1]).unwrap();
    range_wheel.rotate_left(start_pos);

    let mut current_position = 0;
    let (mut current_range, mut reverse) = range_wheel.pop_front().unwrap(); // pop [1,1] to initialize with and ignore counting it
    while current_position < final_position {
        (current_range, reverse) = range_wheel.pop_front().unwrap();
        current_position += current_range[1] - current_range[0] + 1;
    }

    match reverse {  // if the range was added reversely i.e. on the counterclockwise side
        false => {return (current_range[1] - current_position + final_position).to_string()}
        true => {return (current_range[0] + current_position - final_position).to_string()}

    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse::<usize>().unwrap()).collect()
}

fn parse_input_ranges(input: &str) -> Vec<[usize; 2]> {
    input.lines().map(|line| line.split_once('-')
        .map(|num| [num.0.parse::<usize>().unwrap(), num.1.parse::<usize>().unwrap()])
        .unwrap())
        .collect()
}