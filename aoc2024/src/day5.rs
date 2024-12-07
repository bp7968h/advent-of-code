use std::collections::HashMap;
use std::collections::VecDeque;

pub fn run(input: &str) {
   let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in ORDRULES.lines() {
        let mut parts = line.split('|');
        let left = parts.next().unwrap().parse::<usize>().expect("left parse failed");
        let right = parts.next().unwrap().parse::<usize>().expect("right parse failed");
        
        adj_list.entry(left).or_insert_with(|| vec![]).push(right);
    }
    
    let updates: Vec<Vec<usize>> = UPDATES.lines().map(|l| {
        l.split(',').map(|e| e.parse::<usize>().expect("update parse failed")).collect()
    }).collect();
    
    let mid_page_sum: usize = updates.iter().filter_map(|u| {
        if does_follow_rules(&adj_list, &u) {
            let midpoint = u.len() / 2;
            return Some(u[midpoint]);
        }
        None
    }).sum();
    
    println!("Sum: {}", mid_page_sum);
}

fn does_follow_rules(rules: &HashMap<usize, Vec<usize>>, update: &[usize]) -> bool {
    let mut visited: Vec<usize> = Vec::new();
    for u in update {
        if let Some(after_list) = rules.get(&u) {
            for item in after_list {
                if visited.contains(item) {
                    return false
                }
            }
        }
        visited.push(*u);
    }
    true
}
