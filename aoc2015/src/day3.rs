pub fn run_part1(input: &str) -> usize {
    let mut top_down: isize = 0;
    let mut right_left: isize = 0;
    let mut visited: Vec<[isize;2]> = vec![[0,0]];
    
    for c in input.chars() {
        move_and_record(c, &mut top_down, &mut right_left, &mut visited);
    }
    
    println!("[Part1] At least one present: {}", visited.len());
    visited.len()
}

pub fn run_part2(input: &str) -> usize {
    let mut top_down_santa: isize = 0;
    let mut right_left_santa: isize = 0;
    
    let mut top_down_rsanta: isize = 0;
    let mut right_left_rsanta: isize = 0;
    let mut visited: Vec<[isize; 2]> = vec![[0, 0]];
    
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            move_and_record(c, &mut top_down_santa, &mut right_left_santa, &mut visited);
        } else {
            move_and_record(c, &mut top_down_rsanta, &mut right_left_rsanta, &mut visited);
        }
    }
    
    println!("[Part2] At least one present: {}", visited.len());
    visited.len()
}

fn move_and_record(c: char, top_down: &mut isize, right_left: &mut isize, visited: &mut Vec<[isize; 2]>) {
    match c {
        '^' => {
            *top_down += 1;
        },
        'v' => {
            *top_down -= 1;
        },
        '>' => {
            *right_left += 1;
        },
        '<' => {
            *right_left -= 1;
        },
        _ => unreachable!(),
    }
    
    let exists = visited.iter().any(|&item| item == [*top_down, *right_left]);
    
    if !exists {
        visited.push([*top_down, *right_left]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deviler_to_two_houses() {
        let input = ">";
        assert_eq!(run_part1(&input), 2);
    }

    #[test]
    fn deviler_to_two_houses2() {
        let input = "^v";
        assert_eq!(run_part1(&input), 2);
        assert_eq!(run_part2(&input), 3);
    }

    #[test]
    fn devlier_to_four_houses() {
        let input = "^>v<";
        assert_eq!(run_part1(&input), 4);
        assert_eq!(run_part2(&input), 3);
    }

    #[test]
    fn deliver_to_only_two_houses() {
        let input = "^v^v^v^v^v";
        assert_eq!(run_part1(&input), 2);
        assert_eq!(run_part2(&input), 11);
    }
}