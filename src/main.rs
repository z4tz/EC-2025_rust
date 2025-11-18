use ec_2025::{get_inputs, time_solutions};

mod days;


fn main() {
    run_day(11);
    //run_all();
}

fn run_all() {
    for day in 1..21 {
        run_day(day);
    }
}

fn run_day(day: u32) {
    match get_day_solutions(day) {
        Some(solutions) => {
            println!("Solving day {day}");
            time_solutions(solutions, &get_inputs(day));
            println!("-------------------------------------------------------\n");
        }
        None => {println!("No solutions found for day {day}");}
    }

}

fn get_day_solutions(day: u32) -> Option<[fn(&str) -> String; 3]> {
    match day {
        1 => Some(days::day01::get_solutions()),
        2 => Some(days::day02::get_solutions()),
        3 => Some(days::day03::get_solutions()),
        4 => Some(days::day04::get_solutions()),
        5 => Some(days::day05::get_solutions()),
        6 => Some(days::day06::get_solutions()),
        7 => Some(days::day07::get_solutions()),
        8 => Some(days::day08::get_solutions()),
        9 => Some(days::day09::get_solutions()),
        10 => Some(days::day10::get_solutions()),
        11 => Some(days::day11::get_solutions()),
        _ => None,

    }
}
