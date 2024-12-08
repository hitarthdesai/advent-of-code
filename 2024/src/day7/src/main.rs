use std::convert::TryInto;
use std::fs;

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

#[derive(Debug, Clone)]
enum Operator<'a> {
    Add(&'a str),
    Mul(&'a str),
    Concat(&'a str)
}

fn _generate_permutations<Operator: Clone>(len: usize, domain: Vec<Operator>) -> Vec<Vec<Operator>> {
    let num_combinations = domain.len().pow(TryInto::<u32>::try_into(len).unwrap());
    (0..num_combinations).map(move |idx| {
        (0..len)
            .rev()
            .map(|c| {
                let c_value = domain.len().pow(TryInto::<u32>::try_into(c).unwrap());
                let c = (idx / c_value) % domain.len();
                domain[c].clone()
            })
            .collect::<Vec<Operator>>()
    }).collect::<Vec<Vec<Operator>>>()
}

fn part1() -> i64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut ans = 0;
    for line in input.lines() {
        let x = line.split_once(": ").unwrap();
        let expected_result = x.0.parse::<i64>().unwrap();

        let numbers = x.1.split_whitespace().map(|y| y.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let permutations = _generate_permutations(numbers.len() - 1, Vec::from([Operator::Add("+"), Operator::Mul("*")]));

        for p in permutations {
            let mut result = expected_result as f64;
            for (i, op) in p.iter().enumerate() {
                let operand = numbers[numbers.len() - 1 - i] as f64;
                match op {
                    Operator::Add(_) => result -= operand,
                    Operator::Mul(_) => result /= operand,
                    _ => {}
                };

                if result.ceil() != result.floor() {
                    break;
                }
            }

            if result.ceil() != result.floor() {
                continue;
            }

            if result == numbers[0] as f64 {
                ans += TryInto::<i64>::try_into(expected_result).unwrap();
                break;
            }
        }
    }

    ans
}

fn part2() -> i64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut ans = 0;
    for line in input.lines() {
        let x = line.split_once(": ").unwrap();
        let expected_result = x.0.parse::<i64>().unwrap();

        let numbers = x.1.split_whitespace().collect::<Vec<&str>>();
        let permutations = _generate_permutations(numbers.len() - 1, Vec::from([Operator::Add("+"), Operator::Mul("*"), Operator::Concat("|")]));

        let mut strings: Vec<String> = Vec::with_capacity(permutations.len());
        for p in permutations {
            let mut joined_string = numbers[0].to_string();
            for i in 1..numbers.len() {
                joined_string.push_str(match p[i-1] {
                    Operator::Add(s) => s,
                    Operator::Mul(s) => s,
                    Operator::Concat(s) => s,
                });
                joined_string.push_str(&numbers[i]);
            }

            strings.push(joined_string);
        }

        let symbols_regex = regex::Regex::new(r"[+|*]").unwrap();
        for s in strings.iter() {
            let mut result = expected_result as f64;

            let mut s_copy = s.clone();
            while s_copy.len() > 0 && result > 0.0 {
                let str = s_copy.clone();

                let operator_match = symbols_regex.captures_iter(str.as_str()).last();
                match operator_match {
                    Some(caps) => {
                        let operator = caps.iter().last().unwrap().unwrap();
                        let _operand = s_copy[operator.end()..].to_string();
                        let operand = _operand.parse::<i64>().unwrap() as f64;
                        s_copy.replace_range(operator.start().., "");
                        match operator.as_str() {
                            "+" => { result -= operand }
                            "*" => { result /= operand }
                            "|" => {
                                result -= operand;
                                while result.ceil() == result.floor() {
                                    result /= 10.0;
                                }

                                result *= 10.0;
                            }
                            _ => {}
                        }

                        if result.ceil() != result.floor() {
                            break;
                        }
                    }
                    None => {
                        let _str = str.split("|").collect::<Vec<&str>>().join("");
                        result -= _str.parse::<i64>().unwrap() as f64;
                        s_copy.clear();
                    }
                }
            }

            if result.ceil() != result.floor() {
                continue;
            }

            if result == 0.0 {
                ans += TryInto::<i64>::try_into(expected_result).unwrap();
                break;
            }
        }
    }

    ans
}
