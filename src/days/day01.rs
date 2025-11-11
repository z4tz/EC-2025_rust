
pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn parse_input( input: &str ) -> (Vec<&str>, Vec<i32>) {
    let mut lines = input.lines();
    let names = lines.nth(0).unwrap().split(',').collect::<Vec<&str>>();
    let instruction_ammounts = lines.last().unwrap()
        .replace("R","")
        .replace("L","-")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (names, instruction_ammounts)
}

fn part1(input: &str) -> String {
    let (names, instruction_ammounts) = parse_input(input);

    let mut pointer: i32 = 0;
    let max_value = names.len() as i32 - 1;
    for amount in &instruction_ammounts {
        pointer += amount;
        if pointer < 0 { pointer = 0 }
        else if pointer > max_value { pointer = max_value }
    }
    names[pointer as usize].to_string()
}

fn part2(input: &str) -> String {
    let (names, instruction_ammounts) = parse_input(input);
    let name_count = names.len() as i32;

    let pointer:i32 = (instruction_ammounts.iter().sum::<i32>() % name_count + name_count) % name_count;  // do the modulo twice to ensure a positive number

    names[pointer as usize].to_string()
}
fn part3(input: &str) -> String {
    let (mut names, instruction_ammounts) = parse_input(input);
    let name_count = names.len() as i32;
    for amount in &instruction_ammounts {
        let test = (amount % name_count + name_count) % name_count;
        names.swap(0, test as usize);
    }
    names.first().unwrap().to_string()
}
