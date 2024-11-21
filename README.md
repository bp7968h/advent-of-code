# advent-of-code

This repository contains my solutions to the Advent of Code challenges, implemented in Rust. The project is organized as a workspace with each year's challenges in separate crates, and a `aoc-runner` crate to execute specific challenges via a command-line interface (CLI).

## Prerequisites

- Rust (latest stable version)
- Cargo

## Clone and Build the Project

To build the entire workspace, run:

```sh
git clone https://github.com/bp7968h/advent-of-code
cd advent-of-code
cargo build
```

## Running a Specific Challenge

You can use the `aoc-runner` crate to execute a specific challenge. The CLI expects two arguments: the year and the day of the challenge.

```bash
# To run day1, advent of code challenge from the year 2015
cargo run --bin runner -- 2015 day1
```

## Running Test

Every challenge have unit tests written for them, so you can run all the tests from the workspace root as follow:

```bash
# Runs all the test for all the challenges
cargo test
```

## Input Files
Input files for each day’s challenge are stored in the `inputs` directory, organized by year. This path is used by runner to provide the input to the puzzle based on the provided year and day.

## Acknowledgments
- Advent of Code by Eric Wastl
- Rust community for their amazing support and resources

> Happy coding!
