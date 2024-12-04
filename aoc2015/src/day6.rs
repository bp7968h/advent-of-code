

pub fn run(input: &str) {
    let light_grid: &[[usize;1000];1000] = &[[0;1000];1000];
    let formatted_input: Vec<(&str, (usize,usize), (usize,usize))> = input.lines().filter_map(|line| {
        let mut line = line.trim();
        let command = if line.starts_with("turn on") {
            line = line.strip_prefix("turn on").unwrap().trim();
            "on"
        } else if line.starts_with("turn off") {
            line = line.strip_prefix("turn off").unwrap().trim();
            "off"
        } else if line.starts_with("toggle") {
            line = line.strip_prefix("toggle").unwrap().trim();
            "toggle"
        } else {
            panic!("Invalid command");
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 || parts[1] != "through" {
            panic!("Invalid format")
        }

        let first_corner: Vec<usize> = parts[0]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

        let second_corner: Vec<usize> = parts[2]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Some((command, (first_corner[0], first_corner[1]), (second_corner[0], second_corner[1])))
    }).collect();

    for (command, first, second) in formatted_input {
        println!("{:?} {:?} {:?}", command, first, second);
    }
}