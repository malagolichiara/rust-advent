use regex::Regex;
use std::io::{self, BufRead};
use std::{env, fs::File};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = args.get(2).expect("Missing input file path argument");
    let part = args.get(3).unwrap_or(&String::from("part1")).clone();

    let input_file = File::open(input_file_path).expect("Cannot open input file");
    let lines = io::BufReader::new(input_file).lines();
    let values: Vec<String> = lines
        .map(|l| l.expect("Cannot read line from file"))
        .collect();

    match part.as_str() {
        "part1" => assert_eq!(part1(values), 560),
        "part2" => assert_eq!(part2(values), 303),
        _ => panic!("Invalid part"),
    };

    Ok(())
}

struct Line {
    min: u32,
    max: u32,
    char: char,
    password: String,
}

fn parse_line(line: &String) -> Line {
    let re = Regex::new(r"^(\d+)-(\d+)\s([a-zA-Z]):\s([a-zA-Z]+)$").unwrap();
    let capture = re.captures(line).unwrap();

    Line {
        min: capture[1].parse().unwrap(),
        max: capture[2].parse().unwrap(),
        char: capture[3].parse().unwrap(),
        password: capture[4].parse().unwrap(),
    }
}

fn validate_line(line: &Line) -> bool {
    let occurences = line.password.matches(line.char).count() as u32;

    occurences >= line.min && occurences <= line.max
}

fn part1(lines: Vec<String>) -> u32 {
    lines.iter().map(parse_line).filter(validate_line).count() as u32

    // let mut valid_count = 0;
    // for line in &lines {
    //     let valid = validate_line(parse_line(line));
    //     if valid {
    //         valid_count += 1;
    //     }
    // }
    // valid_count

    // let mut valid_count = 0;
    // for line in &lines {
    //     match validate_line(&parse_line(line)) {
    //         true => valid_count += 1,
    //         false => (),
    //     }
    // }
    // valid_count
}

fn part2(lines: Vec<String>) -> u32 {
    parse_line(lines.get(0).unwrap());
    0
}
