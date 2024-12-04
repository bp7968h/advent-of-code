// all increasing or decreasing
// two adjacent levels differ by at least one and at most three
// difference must be 1 or 2 or 3
// if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

pub fn run(input: &str) {
    let input: Vec<Vec<usize>> = input.lines().map(|l| {
        l.split_whitespace()
        .filter_map(|i| i.parse::<usize>().ok())
        .collect()
    }).collect();
    
    let mut safe_count = 0;
    for i in input.iter() {
        if is_safe(&i) {
            safe_count += 1;
            // println!("Safe: {:?} {}", i, position);
        } else {
            // println!("Unsafe: {:?} {}", i, position);
            for j in 0..i.len() {
                let mut temp = i.clone();
                let _ = temp.remove(j);
                
                if is_safe(&temp) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    
    println!("Safe Count: {}", safe_count)
}

fn is_safe(slice: &[usize]) -> bool {
    let mut order: Option<&str> = None;
    for i in 0..slice.len() - 1 {
        let pointer_one = slice[i];
        let pointer_two = slice[i+1];
        let diff;
        
        if pointer_one < pointer_two {
            if i == 0 {
                order = Some("inc");
            } else {
                if order != Some("inc") {
                    return false;
                }
            }
            diff = pointer_two - pointer_one;
        } else if pointer_one > pointer_two {
            if i == 0 {
                order = Some("dec");
            } else {
                if order != Some("dec") {
                    return false;
                }
            }
            diff = pointer_one - pointer_two;
        } else {
            return false;
        }
        
        match diff {
            1 | 2 | 3 => (),
            _ => return false
        }
    }
    true
}