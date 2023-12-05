use std::{collections::HashMap, fs};

enum StartPoint {
    First,
    Last,
}
fn main() {
    fn get_number(start_point: StartPoint, line: &str) -> u32 {
        let numbers: HashMap<&str, u32> = HashMap::from([
            ("nine", 9),
            ("eight", 8),
            ("seven", 7),
            ("six", 6),
            ("five", 5),
            ("four", 4),
            ("three", 3),
            ("two", 2),
            ("one", 1),
        ]);
        for lenght in 1..=line.len() {
            let sub: &str = match start_point {
                StartPoint::First => &line[..lenght],
                StartPoint::Last => &line[line.len() - lenght..line.len()],
            };
            let added_char = match start_point {
                StartPoint::First => sub.chars().last().unwrap().to_digit(10),
                StartPoint::Last => sub.chars().next().unwrap().to_digit(10),
            };
            if added_char.is_some() {
                return added_char.unwrap();
            }
            for number in numbers.keys() {
                if sub.contains(number) {
                    return *numbers.get(number).unwrap();
                }
            }
        }
        0
    }

    fn parse_line(line: &str) -> u32 {
        let _numbers: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digits: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap_or(0)
    }
    fn parse_line2(line: &str) -> u32 {
        let first = get_number(StartPoint::First, line);
        let last = get_number(StartPoint::Last, line);
        format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
    }

    let contents = fs::read_to_string("../input/01.txt").expect("YOU DID SOMETHING WRONG");
    let sum_part_one: u32 = contents.lines().map(parse_line).sum();
    let sum_part_two: u32 = contents.lines().map(parse_line2).sum();

    println!("Part one: {}", sum_part_one);
    println!("Part two: {}", sum_part_two);
}
