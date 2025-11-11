pub fn get_solutions() -> [fn(&str) -> String; 3] {
    [part1, part2, part3]
}

fn part1(input: &str) -> String {
    let (identifier, numbers) = &parse_input(input)[0];
    Sword::new(*identifier,numbers).quality.to_string()
}

fn part2(input: &str) -> String {
    let inputs = parse_input(input);
    let swords = inputs.iter()
        .map(|(identifier, numbers)| Sword::new(*identifier, numbers))
        .collect::<Vec<_>>();
    let qualities: Vec<i64> = swords.iter()
        .map(|sword| sword.quality).collect();
    (qualities.iter().max().unwrap() - qualities.iter().min().unwrap()).to_string()
}

fn part3(input: &str) -> String {
    let inputs = parse_input(input);
    let mut swords = inputs.iter()
        .map(|(identifier, numbers)| Sword::new(*identifier, numbers))
        .collect::<Vec<_>>();
    swords.sort_by( |a, b| {
        if a.quality != b.quality {
            return a.quality.cmp(&b.quality)
        }

        // if quality equal, compare by each segment
        for (segment_a, segment_b) in a.segment_numbers.iter().zip(b.segment_numbers.iter()) {
            if segment_a != segment_b {
                return segment_a.cmp(&segment_b)
            }
        }

        // if all segments equal, compare by identifier
        return a.identifier.cmp(&b.identifier);
    });
    let checksum = swords.iter().rev().enumerate()
        .fold(0, |acc, (i, sword)| {
            acc + ((i+1) * sword.identifier as usize)
        });

    checksum.to_string()
}

fn parse_input(input: &str) -> Vec<(i32, Vec<i32>)> {
    input.lines().map(|line| {
        let parts = line.split_once(':').unwrap();
        let identifier = parts.0.parse::<i32>().unwrap();
        let numbers = parts.1.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        (identifier, numbers)
    }).collect()
}

struct Sword {
    identifier: i32,
    segment_numbers: Vec<i32>,
    quality: i64,
}

impl Sword {
    fn new(identifier: i32, numbers: &Vec<i32>) -> Self {
        //generate spine segments
        let mut segments: Vec<Segment> = vec![];
        for number in numbers {
            let mut was_added = false;
            for segment in segments.iter_mut() {
                if segment.try_add(*number) {
                    was_added = true;
                    break
                }
            }
            if !was_added {
                segments.push(Segment::new(*number));
            }
        }
        let segment_numbers = segments.iter().map(|segment| segment.to_i32()).collect();

        let quality: i64 = segments.iter().rev().enumerate()
            .fold(0, |acc, (i, segment)| acc + segment.get_spine() as i64 * 10_i64.pow(i as u32));


        Sword {
            identifier,
            segment_numbers,
            quality
        }
    }
}

struct Segment {
    numbers: [i32; 3]
}
impl Segment {
    fn new( number: i32 ) -> Segment {
        Segment { numbers: [0, number, 0] }
    }
    fn try_add(&mut self, number: i32) -> bool{
        if self.numbers[0] == 0 && number < self.numbers[1] {
            self.numbers[0] = number;
            true
        }
        else if self.numbers[2] == 0 && number > self.numbers[1] {
            self.numbers[2] = number;
            true
        }
        else {
            false
        }
    }
    fn get_spine(&self) -> i32 {
        self.numbers[1]
    }
    fn to_i32(&self) -> i32 {
        self.numbers.iter().rev().enumerate().rev()  // double rev to handle acc/10 correctly
            .fold(0, |acc, (i, number)| {
                match number {
                    &0 => acc/10, // if number is a 0 it should be skipped
                    number => acc + number * 10_i32.pow(i as u32)
                }
            })
    }
}