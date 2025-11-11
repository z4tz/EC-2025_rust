use std::fs;
use std::time::Instant;
use std::env::var;

pub fn get_inputs(day: u32) -> [Option<String>; 3] {
    let mut inputs: [Option<String>; 3] = Default::default();
    let basepath = var("EC2025-PATH").unwrap();
    for i in 1.. 4 {
        match fs::read_to_string(&format!(r"{basepath}\inputs\day{day}\{i}.txt")) {
            Ok(data) => {
                if !data.is_empty() {
                    inputs[i-1] = Some(data)
                }
            }
            Err(e) => {
                println!("Error reading inputs: {}", e);
            }
        }
    }
    inputs
}

pub fn time_solutions(functions: [fn(&str) -> String; 3], inputs: &[Option<String>; 3]) {
    let start = Instant::now();
    functions.iter().zip(inputs).enumerate().for_each(|(index, (function, input))| {
        match input {
            Some(input) => {
                let part_start = Instant::now();
                println!("Part {} answer: {}", index+1, function(input));
                println!("Time for part: {:?}\n", part_start.elapsed());

            }
            None => {println!("No input found for solution {}", index + 1)}
        }
    });
    println!("Time for all parts {:?}", start.elapsed());
}