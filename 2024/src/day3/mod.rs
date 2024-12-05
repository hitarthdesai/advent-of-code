use std::fs;
use regex::Regex;

#[allow(dead_code)]
fn main() {
    let mut args = std::env::args();
    args.next(); /* Skip the first argument */
    let part = args.next().unwrap().parse::<u8>().unwrap();

    match part {
        1 => {
            let result = part1();
            println!("Part {part}: {result}")
        },
        2 => {
            let result = part2();
            println!("Part {part}: {result}")
        },
        _ => { eprintln!("Invalid part {part}") }
    }
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = regex.captures_iter(&input);

    matches.map(|m| m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap()).sum()
}

fn part2() -> i32 {
    return 0
}