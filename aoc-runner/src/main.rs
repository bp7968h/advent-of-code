use std::{env, process};

use aoc2015;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        3 => {
            let year = &args[1];
            let day = &args[2];

            validate_year(year);
            validate_day(day);
            
            match year.as_str() {
                "2015" => {
                    match day.as_str() {
                        "day1" => {
                            match get_input_file(year, day) {
                                Ok(puzzle_input) => {
                                    let _ = aoc2015::day1::run(&puzzle_input);
                                },
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                    process::exit(1);
                                }
                            }
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        },
        _ => {
            const ERR_MSG: &str = r#"
                Usage: runner <year> day<day number>
                Eg: To run the code for day 1 of 2015 AoC puzzle:
                    runner 2015 day1
            "#;
            eprintln!("{ERR_MSG}");
            process::exit(1);
        }
    }
}

fn validate_year(year: &str) {
    let valid_years = ["2015", "2016", "2017", "2018", "2019", "2020", "2021", "2023"];
    if !valid_years.contains(&year) {
        eprintln!("Invalid year: {}", year);
        eprintln!("AoC started from 2015 till present. Use one of these year");
        eprintln!("\t{:?}", valid_years);
        process::exit(1);
    }
}

fn validate_day(day: &str) {
    if !day.starts_with("day") || day.len() < 4 {
        eprintln!("Invalid day: {}", day);
        eprintln!("Day must be in the format `day<number>` (e.g., day1 to day25).");
        process::exit(1);
    }

    if let Ok(day_num) = day[3..].parse::<u32>() {
        if !(1..=25).contains(&day_num) {
            eprintln!("Invalid day number: {}", day_num);
            eprintln!("Day number must be between 1 and 25.");
            process::exit(1);
        }
    } else {
        eprintln!("Day number `{}` is not a valid number.", &day[3..]);
        eprintln!("Day number must be between 1 and 25.");
        process::exit(1);
    }
}

fn get_input_file(year: &str, day: &str) -> std::io::Result<String> {
    let path = format!("inputs/aoc{}/{}.txt", year, day);
    std::fs::read_to_string(&path)
}
