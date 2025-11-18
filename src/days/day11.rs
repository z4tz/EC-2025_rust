pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let mut numbers = parse_input(input);
    let mut checksums = vec![calc_checksum(&numbers)];
    while pass_forward(&mut numbers) {
        checksums.push(calc_checksum(&numbers));
    }
    while pass_back(&mut numbers) {
        checksums.push(calc_checksum(&numbers));
    }
    checksums[10].to_string()
}

fn part2(input: &str) -> String {
    let mut numbers = parse_input(input);
    let mut round = 0;
    while pass_forward(&mut numbers) {
        round += 1;
    }
    while pass_back(&mut numbers) {
        round += 1;
    }
    round.to_string()
}

fn part3(input: &str) -> String {
    // this relies on that the input is strictly increasing which part 3 is for some reason.
    let numbers = parse_input(input);
    let mean: i64 = numbers.iter().sum::<i64>()/numbers.len() as i64;
    let steps: i64 = numbers.iter().filter(|num| mean > **num).map(|num| mean - num).sum();
    steps.to_string()
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn pass_forward(numbers: &mut Vec<i64>) -> bool {
    let mut changed = false;
    for i in 0..numbers.len() - 1 {
        if numbers[i] > numbers[i + 1] {
            numbers[i] -= 1;
            numbers[i + 1] += 1;
            changed = true;
        }
    }
    changed
}

fn pass_back(numbers: &mut Vec<i64>) -> bool {
    let mut changed = false;
    for i in 0..numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            numbers[i] += 1;
            numbers[i + 1] -= 1;
            changed = true;
        }
    }
    changed
}

fn calc_checksum(numbers: &Vec<i64>) -> usize {
    numbers.iter().enumerate().fold(0, |acc, (i, x)| {acc + (i + 1) * (*x as usize)})
}