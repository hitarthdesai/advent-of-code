use std::fs;

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
    let input = fs::read_to_string("./src/day2/input.txt").unwrap();
    let mut safe_counter = 0;

    for line in input.lines() {

        let nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let is_safe = are_levels_safe(nums.clone());

        if is_safe {
            safe_counter += 1;
        }
    }

    safe_counter
}

fn part2() -> i32 {
    let input = fs::read_to_string("./src/day2/input.txt").unwrap();
    let mut safe_counter = 0;

    for line in input.lines() {
        let nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let is_original_safe = are_levels_safe(nums.clone());
        match is_original_safe {
            true => {
                safe_counter += 1;
            },
            false => {
                for i in 0..nums.len() {
                    let mut new_nums = nums.clone();
                    new_nums.remove(i);

                    let is_new_nums_safe = are_levels_safe(new_nums);
                    if is_new_nums_safe {
                        safe_counter += 1;
                        break;
                    }
                }
            }
        }
    }

    safe_counter
}

enum SortingDirection {
    Increasing,
    Decreasing
}

fn are_levels_safe(nums: Vec<i32>) -> bool {
    let nums_count = nums.len();

    let mut sorting_order: Option<SortingDirection> = None;
    for i in 0..nums_count-1 {
        let diff = nums[i+1] - nums[i];

        let is_okay= match sorting_order {
            None => {
                let mut _is_okay = false;
                if diff >= 1 && diff <= 3 {
                    sorting_order = Some(SortingDirection::Increasing);
                    _is_okay = true
                } else if diff >= -3 && diff <= -1 {
                    sorting_order = Some(SortingDirection::Decreasing);
                    _is_okay = true
                }

                _is_okay
            },
            Some(SortingDirection::Increasing) => {
                diff >= 1 && diff <= 3
            },
            Some(SortingDirection::Decreasing) => {
                diff >= -3 && diff <= -1
            }
        };

        if !is_okay {
            return false;
        }
    }

    true
}