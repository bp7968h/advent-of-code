use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        3 => {
            let year = &args[1];
            let day = &args[2];
            let path = format!("../inputs/aoc{}/{}", year, day);

            match year.as_str() {
                "2015" => {
                    match day.as_str() {
                        "day1" => {
                            println!("2015 day1");
                        },
                        _ => {
                            println!("Invalid day: {}", day);
                            println!("Advent of Code runs from day1 to day25");
                        }
                    }
                },
                _ => {
                    println!("Invalid year: {}", year);
                    println!("Advent of Code started from 2015 so,");
                    println!("Use one of these year");
                    println!("\t[2015, 2016, 2017, 2018, 2019, 2020, 2021, 2023]");
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
