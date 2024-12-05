mod day1;
mod day2;
mod day3;

enum Action {
    Run(u8, u8),
    Init(u8),
}

fn parse_arguments() -> Result<Action, String> {
    let mut arguments = std::env::args();
    arguments.next(); /* Skip the first argument */

    match arguments.len() {
        2 => {
            let first_arg = arguments.next().ok_or("Must have at least one argument")?;
            match first_arg.as_str() {
                "--init" => {
                    let day = arguments.next().ok_or("Must have at least two arguments")?.parse::<u8>().expect("Second argument is not a number");
                    Ok(Action::Init(day))
                },
                _ => {
                    let day = first_arg.parse::<u8>().map_err(|err| err.to_string())?;
                    let part = arguments.next().ok_or("Must have at least two arguments")?.parse::<u8>().map_err(|err| err.to_string())?;
                    Ok(Action::Run(day, part))
                }
            }
        },
        _ => { Err("Wrong number of arguments".to_string()) }
    }
}

fn main() {
    match parse_arguments() {
        Ok(Action::Init(day)) => {
            let _path = format!("src/day{}", day);
            let path = std::path::Path::new(&_path);
            std::fs::create_dir(path).expect(format!("Could not create directory for day {}", day).as_str());
            std::env::set_current_dir(path).expect(format!("Could not set directory to day {}", day).as_str());
        },
        Ok(Action::Run(day, part)) => {
            match day {
                1 => { day1::main(part) },
                2 => { day2::main(part) },
                3 => { day3::main(part) },
                _ => { println!("Day {} hasn't been implemented yet", day); }
            }
        },
        Err(message) => { eprintln!("{}", message); }
    };
}