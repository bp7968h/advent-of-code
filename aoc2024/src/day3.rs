// digit = 1..=3
// mul(digit,digit)
pub fn run(input: &str) {
    let part_one_ans = part_one_multiplication(input);
    println!("Part one puzzle answer: {part_one_ans}");

    let part_two_ans = part_two_multiplication(input);
    println!("Part two puzzle answer: {part_two_ans}");
}

fn part_one_multiplication(input: &str) -> usize {
    let mut input_iter = input.chars().peekable();
    let mut extracted: Vec<String>= vec![];
    
    while let Some(character) = input_iter.next() {
        match character {
            'm' if input_iter.peek() == Some(&'u') => {
                let _ = input_iter.next();
                if input_iter.peek() == Some(&'l') {
                    let _ = input_iter.next();
                    if input_iter.peek() == Some(&'(') {
                        let _ = input_iter.next();
                        let mut number_str = String::new();
                        while let Some(t) = input_iter.peek() {
                            match t {
                                '0'..='9' => {
                                    number_str.push(*t);
                                    let _ = input_iter.next();
                                },
                                ',' => {
                                    if number_str.is_empty() || number_str.len() > 3 {
                                        break;
                                    }
                                    number_str.push(*t);
                                    let _ = input_iter.next();
                                },
                                _ => break
                            }
                        }
                        if let Some(cb) = input_iter.peek() {
                            match cb {
                                ')' => {
                                    let _ = input_iter.next();
                                    extracted.push(number_str);
                                },
                                _ => continue
                            }
                        }
                    }
                }
            },
            _ => continue
        }
    }

    let total: usize = extracted.iter().map(|s| {
        let mut parts = s.split(',');
        let left = parts.next().unwrap().parse::<usize>().expect("failed to parse left");
        let right = parts.next().unwrap().parse::<usize>().expect("failed to parse right");
        
        left * right
    }).sum();

    total
}

#[derive(Debug, PartialEq)]
enum Instructions {
    Do,
    Dont,
}

fn part_two_multiplication(input: &str) -> usize {
    let mut input_iter = input.chars().peekable();
    let mut last_instruction: Option<Instructions> = None;
    let mut extracted: Vec<String>= vec![];
    
    while let Some(character) = input_iter.next() {
        match character {
            'm' if input_iter.peek() == Some(&'u') => {
                let _ = input_iter.next();
                if input_iter.peek() == Some(&'l') {
                    let _ = input_iter.next();
                    if input_iter.peek() == Some(&'(') {
                        let _ = input_iter.next();
                        let mut number_str = String::new();
                        while let Some(t) = input_iter.peek() {
                            match t {
                                '0'..='9' => {
                                    number_str.push(*t);
                                    let _ = input_iter.next();
                                },
                                ',' => {
                                    if number_str.is_empty() || number_str.len() > 3 {
                                        break;
                                    }
                                    number_str.push(*t);
                                    let _ = input_iter.next();
                                },
                                _ => break
                            }
                        }
                        if let Some(cb) = input_iter.peek() {
                            match cb {
                                ')' => {
                                    let _ = input_iter.next();
                                    match last_instruction {
                                        None | Some(Instructions::Do) => extracted.push(number_str),
                                        Some(Instructions::Dont) => continue
                                    }
                                },
                                _ => continue
                            }
                        }
                    }
                }
            },
            'd' if input_iter.peek() == Some(&'o') => {
                let _ = input_iter.next();
                match input_iter.peek(){
                    Some(&'(') | Some(&'n') => {
                        let _ = input_iter.next();
                        match input_iter.peek() {
                            Some(&')') => {
                                let _ = input_iter.next();
                                last_instruction = Some(Instructions::Do);
                            },
                            Some(&'\'') => {
                                let _ = input_iter.next();
                                if input_iter.peek() == Some(&'t') {
                                    let _ = input_iter.next();
                                    if input_iter.peek() == Some(&'(') {
                                        let _ = input_iter.next();
                                        if input_iter.peek() == Some(&')') {
                                            let _ = input_iter.next();
                                            last_instruction = Some(Instructions::Dont);
                                        }
                                    }
                                }
                            },
                            _ => continue
                        }
                    }
                    _ => continue
                }
            },
            _ => continue
        }
    }

    let total: usize = extracted.iter().map(|s| {
        let mut parts = s.split(',');
        let left = parts.next().unwrap().parse::<usize>().expect("failed to parse left");
        let right = parts.next().unwrap().parse::<usize>().expect("failed to parse right");
        
        left * right
    }).sum();

    total
}