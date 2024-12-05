pub fn run(input: &str) {
    // read rules
    let rules: &str = r#""#;
    let ord_rules: Vec<(usize, usize)> = rules.lines().map(|o| {
        let mut parts = o.split('|');
        let left = parts.next().unwrap().parse::<usize>().expect("ord left parse failed");
        let right = parts.next().unwrap().parse::<usize>().expect("ord right parse failed");
        (left, right)
    }).collect();
    
    let updates: Vec<Vec<usize>> = input.lines().map(|u| {
        u.split(',').map(|i| i.parse::<usize>().expect("update parse failed")).collect()
    }).collect();
    
    
}
