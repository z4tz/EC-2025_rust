pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let numbers = parse_input(input);
    let first = *numbers.first().unwrap();
    let last = *numbers.last().unwrap();
    (first / last  * 2025.0).round().to_string()
}

fn part2(input: &str) -> String {
    let numbers = parse_input(input);
    let first = *numbers.first().unwrap();
    let last = *numbers.last().unwrap();
    (last / first  * 10_000_000_000_000.0).ceil().to_string()
}

fn part3(input: &str) -> String {
    let (first, combined_factor, last) = parse_input_alternative(input);
    ((first / last * combined_factor * 100.0) as i64).to_string()
}

fn parse_input(input: &str) -> Vec<f64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn parse_input_alternative(input: &str) -> (f64, f64, f64) {
    let mut iterator = input.lines();
    let first = iterator.next().unwrap().parse().unwrap();

    let combined_factor = iterator.take(input.lines().count() - 2)
        .fold(1.0, |acc, line| {
            let parts = line.split_once('|').unwrap();
            parts.1.parse::<f64>().unwrap() / parts.0.parse::<f64>().unwrap() * acc
        });

    let last = input.lines().last().unwrap().parse::<f64>().unwrap();

    (first, combined_factor, last)
}