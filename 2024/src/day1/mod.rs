use std::collections::HashMap;
use std::fs;
use std::iter::zip;

pub fn main(part: u8) {
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
    let input = fs::read_to_string("./src/day1/input.txt").unwrap();
    let num_lines = input.lines().count();

    let mut vec1 = Vec::with_capacity(num_lines);
    let mut vec2 = Vec::with_capacity(num_lines);

    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        vec1.push(nums.next().unwrap());
        vec2.push(nums.next().unwrap());
    }

    vec1.sort_by(|a, b| b.cmp(a));
    vec2.sort_by(|a, b| b.cmp(a));

    zip(vec1, vec2).map(|(a, b)| (a - b).abs()).sum()
}

fn part2() -> i32 {
    let input = fs::read_to_string("./src/day1/input.txt").unwrap();
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let num1 = nums.next().unwrap();

        match map.contains_key(&num1) {
            true => {
                let (left_count, _) = map.get_mut(&num1).unwrap();
                *left_count += 1;
            },
            false => {
                map.insert(num1, (1, 0));
            }
        }

        let num2 = nums.next().unwrap();
        match map.contains_key(&num2) {
            true => {
                let (_, right_count) = map.get_mut(&num2).unwrap();
                *right_count += 1;
            },
            false => {
                map.insert(num2, (0, 1));
            }
        }
    }

    map.iter().map(|(x1, (x2, x3))| { x1 * x2 * x3 }).sum::<i32>()
}