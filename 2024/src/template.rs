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
    /** Code Here */
}

fn part2() -> i32 {
    let input = fs::read_to_string("./src/day1/input.txt").unwrap();
    /** Code Here */
}