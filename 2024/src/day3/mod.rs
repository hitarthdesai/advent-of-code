use std::fs;
use regex::Regex;

pub fn main() {
    let p1 = part1();
    println!("{}", p1);
    // let p2 = part2();
    // println!("{}", p2);
}

// regex to capture numbers from pattern mul(<number>,<number>)

fn part1() -> i32 {
    let input = fs::read_to_string("./src/day3/input.txt").unwrap();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = regex.captures_iter(&input);

    matches.map(|m| m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap()).sum()
}

// fn part2() -> i32 {
//     let input = fs::read_to_string("./src/day3/input.txt").unwrap();
//     let mut safe_counter = 0;
//
//     for line in input.lines() {
//         let nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
//         let is_original_safe = are_levels_safe(nums.clone());
//         match is_original_safe {
//             true => {
//                 safe_counter += 1;
//             },
//             false => {
//                 for i in 0..nums.len() {
//                     let mut new_nums = nums.clone();
//                     new_nums.remove(i);
//
//                     let is_new_nums_safe = are_levels_safe(new_nums);
//                     if is_new_nums_safe {
//                         safe_counter += 1;
//                         break;
//                     }
//                 }
//             }
//         }
//     }
//
//     safe_counter
// }