enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "right" => Direction::Right,
            "bottom" => Direction::Bottom,
            "left" => Direction::Left,
            "top" => Direction::Top,
            _ => panic!("Cannot parse"),
        }
    }
}

fn can_check_part1(direction: Direction, position: (usize, usize), x: &usize, y: &usize) -> bool {
    match direction {
        Direction::Right => (y - position.1) > 3,
        Direction::Bottom => (x - position.0) > 3,
        Direction::Left => (y - (y - position.1)) >= 3 ,
        Direction::Top => (x - (x - position.0)) >= 3,
    }
}

fn run_part1(input: &str) {
 let mut xmas_count = 0;
    let formatted: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let x_len = formatted.len();
    for x_idx in 0..x_len {
        let y_len = formatted[x_idx].len();
        for y_idx in 0..y_len {
            match formatted[x_idx][y_idx] {
                'X' => {
                    let right_check = can_check_part1("right".into(), (x_idx, y_idx), &x_len, &y_len);
                    let bottom_check = can_check_part1("bottom".into(), (x_idx, y_idx), &x_len, &y_len);
                    let left_check = can_check_part1("left".into(), (x_idx, y_idx), &x_len, &y_len);
                    let top_check = can_check_part1("top".into(), (x_idx, y_idx), &x_len, &y_len);
                    
                    if right_check {
                        if (formatted[x_idx][y_idx + 1] == 'M') && (formatted[x_idx][y_idx + 2] == 'A') && (formatted[x_idx][y_idx + 3] == 'S') {
                            // println!("{}:{}", x_idx, y_idx);
                            xmas_count += 1;
                        }
                    }
                    
                    if left_check {
                        
                        if (formatted[x_idx][y_idx - 1] == 'M') && (formatted[x_idx][y_idx - 2] == 'A') && (formatted[x_idx][y_idx - 3] == 'S') {
                            // println!("{}:{}", x_idx, y_idx);
                            xmas_count += 1;
                        }
                    }
                    
                    if bottom_check {
                        if (formatted[x_idx + 1][y_idx] == 'M') && (formatted[x_idx + 2][y_idx] == 'A') && (formatted[x_idx + 3][y_idx] == 'S') {
                            // println!("{}:{}", x_idx, y_idx);
                            xmas_count += 1;
                        }
                        
                        if right_check {
                            if (formatted[x_idx + 1][y_idx + 1] == 'M') && (formatted[x_idx + 2][y_idx + 2] == 'A') && (formatted[x_idx + 3][y_idx + 3] == 'S') {
                                // println!("{}:{}", x_idx, y_idx);
                                xmas_count += 1;
                            }
                        }
                        
                        if left_check {
                            if (formatted[x_idx + 1][y_idx - 1] == 'M') && (formatted[x_idx + 2][y_idx - 2] == 'A') && (formatted[x_idx + 3][y_idx - 3] == 'S') {
                                // println!("{}:{}", x_idx, y_idx);
                                xmas_count += 1;
                            }
                        }
                    }
                    
                    if top_check {
                        if (formatted[x_idx - 1][y_idx] == 'M') && (formatted[x_idx - 2][y_idx] == 'A') && (formatted[x_idx - 3][y_idx] == 'S') {
                            // println!("{}:{}", x_idx, y_idx);
                            xmas_count += 1;
                        }
                        
                        if left_check {
                            if (formatted[x_idx - 1][y_idx - 1] == 'M') && (formatted[x_idx - 2][y_idx - 2] == 'A') && (formatted[x_idx - 3][y_idx - 3] == 'S') {
                                // println!("{}:{}", x_idx, y_idx);
                                xmas_count += 1;
                            }
                        }
                        if right_check {
                            if (formatted[x_idx - 1][y_idx + 1] == 'M') && (formatted[x_idx - 2][y_idx + 2] == 'A') && (formatted[x_idx - 3][y_idx + 3] == 'S') {
                                // println!("{}:{}", x_idx, y_idx);
                                xmas_count += 1;
                            }
                        }
                    }
                },
                _ => continue
            }
        }
    }
    println!("{:?}", xmas_count); 
}


fn run_part2(input: &str) {
let mut xmas_count = 0;
    let formatted: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let x_len = formatted.len();
    for x_idx in 0..x_len {
        let y_len = formatted[x_idx].len();
        for y_idx in 0..y_len {
            match formatted[x_idx][y_idx] {
                'A' => {
                    let right_check = can_check_part2("right".into(), (x_idx, y_idx), &x_len, &y_len);
                    let bottom_check = can_check_part2("bottom".into(), (x_idx, y_idx), &x_len, &y_len);
                    let left_check = can_check_part2("left".into(), (x_idx, y_idx), &x_len, &y_len);
                    let top_check = can_check_part2("top".into(), (x_idx, y_idx), &x_len, &y_len);
                    
                    // top left = formatted[x_idx - 1][y_idx - 1] 
                    // top right = formatted[x_idx - 1][y_idx + 1] 
                    // bot left = formatted[x_idx + 1][y_idx - 1] 
                    // bot right = formatted[x_idx + 1][y_idx + 1] 
                    
                    if  (top_check && left_check) && (top_check && right_check) && (right_check && bottom_check) && (bottom_check && left_check) {
                        if (formatted[x_idx - 1][y_idx - 1] == 'M' && formatted[x_idx + 1][y_idx + 1] == 'S') || (formatted[x_idx - 1][y_idx - 1] == 'S' && formatted[x_idx + 1][y_idx + 1] == 'M') {
                            if (formatted[x_idx - 1][y_idx + 1] == 'M' && formatted[x_idx + 1][y_idx - 1] == 'S') || (formatted[x_idx - 1][y_idx + 1] == 'S' && formatted[x_idx + 1][y_idx - 1] == 'M') {
                                xmas_count += 1;
                            }
                        }
                    }
                },
                _ => continue
            }
        }
    }
    println!("{:?}", xmas_count);
}

fn can_check_part2(direction: Direction, position: (usize, usize), x: &usize, y: &usize) -> bool {
    match direction {
        Direction::Right => (y - position.1) > 1,
        Direction::Bottom => (x - position.0) > 1,
        Direction::Left => (y - (y - position.1)) >= 1 ,
        Direction::Top => (x - (x - position.0)) >= 1,
    }
}