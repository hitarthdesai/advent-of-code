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
            let _path = format!("src/day{}", day);
            let path = std::path::Path::new(&_path);
            std::env::set_current_dir(path).expect(format!("Could not set directory to day {}", day).as_str());

            let compilation_output = std::process::Command::new("rustc")
                .arg("mod.rs")
                .arg("-o")
                .arg("output")
                .output()
                .expect("Could not execute rustc");

            if !compilation_output.status.success() {
                eprintln!("Failed to compile: {}", String::from_utf8_lossy(&compilation_output.stderr));
                std::process::exit(1);
            }

            let output = std::process::Command::new("./output")
                .arg(part.to_string())
                .output()
                .expect("Could not run executable");

            if !output.status.success() {
                eprintln!("Failed to run executable: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }

            println!("{}", String::from_utf8_lossy(&output.stdout));
        },
        Err(message) => { eprintln!("{}", message); }
    };
}