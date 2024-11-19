use std::{env, process, fs};

use aoc2015;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        3 => {
            let year = &args[1];
            let day = &args[2];
            let path = format!("../inputs/aoc{}/{}.txt", year, day);
            let puzzle_input = if let Ok(content) = fs::read_to_string(&path) {
                    content
                } else {
                    eprintln!("File `{}` not found.", &path);
                    process::exit(1);
            };

            match year.as_str() {
                "2015" => {
                    match day.as_str() {
                        "day1" => {
                            aoc2015::day1::run(&puzzle_input);
                        },
                        _ => {
                            eprintln!("Invalid day: {}", day);
                            eprintln!("Advent of Code runs from day1 to day25");
                            process::exit(1);
                        }
                    }
                },
                _ => {
                    eprintln!("Invalid year: {}", year);
                    eprintln!("Advent of Code started from 2015 so,");
                    eprintln!("Use one of these year");
                    eprintln!("\t[2015, 2016, 2017, 2018, 2019, 2020, 2021, 2023]");
                    process::exit(1);
                }
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
