pub fn run(input: &str) {
    let mut curr_pos: isize = 0;
    let mut has_entered_basement: bool = false;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => curr_pos += 1,
            ')' => curr_pos -= 1,
            _ => panic!("Invalid input"),
        }

        if !has_entered_basement && curr_pos == (-1 as isize) {
            let basement_pos = i + 1;
            println!("First entered basement at position `{:?}`", basement_pos);
            has_entered_basement = true;
        }
    }
    println!("Final position: {}", curr_pos);
}
