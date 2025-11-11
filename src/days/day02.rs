use std::fmt::Display;
use std::ops::{Add, Div, Mul};

pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let mut value = Complex::default();
    let sample = parse_input(input);
    for _ in 0..3 {
        value = value * value;
        value = value / Complex::new(10, 10);
        value = value + sample;
    }
    value.to_string()
}
fn part2(input: &str) -> String {
    let mut count = 0;
    let start_point = parse_input(input);
    for y in 0..101 {
        for x in 0..101 {
            let current = Complex::new(start_point.real + x * 10, start_point.imag + y * 10);
            if engraving_successful(current) { count += 1;}
        }
    }
    count.to_string()
}

fn part3(input: &str) -> String {
    let mut count = 0;
    let start_point = parse_input(input);
    for y in 0..1001 {
        for x in 0..1001 {
            let current = Complex::new(start_point.real + x, start_point.imag + y);
            if engraving_successful(current) { count += 1;}
        }
    }
    count.to_string()
}

fn parse_input(input: &str) -> Complex {
    let test:Vec<i64> = input[3..input.len() - 1].split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    Complex::new(test[0], test[1])
}

fn engraving_successful(position: Complex) -> bool {
    let mut result = Complex::default();
    for _ in 0..100 {
        result = result * result;
        result = result / Complex::new(100_000, 100_000);
        result = result + position;
        if result.real.abs() > 1_000_000 || result.imag.abs() > 1_000_000 {
            return false;
        }
    }
    true
}

#[derive(Default, Clone, Copy, Debug)]
struct Complex {
    real: i64,
    imag: i64
}
impl Complex {
    fn new(real: i64, imag: i64) -> Complex {
        Complex { real, imag }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex { real: self.real + rhs.real, imag: self.imag + rhs.imag }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real / rhs.real,
            imag: self.imag / rhs.imag
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.real, self.imag)
    }
}