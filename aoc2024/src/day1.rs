use std::collections::HashMap;

pub fn run(input: &str) {
    let mut left_v: Vec<usize> = vec![];
    let mut right_v: Vec<usize> = vec![];
    
    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        
        let (Some(left), Some(right)) = (line_iter.next(), line_iter.next()) else {
            panic!("Wrong format");
        };
        
        left_v.push(left.parse::<usize>().unwrap());
        right_v.push(right.parse::<usize>().unwrap());
    }
    assert_eq!(left_v.len(), right_v.len());


    let mut records: HashMap<usize, usize> = HashMap::new();
    let mut sum: usize = 0;
    
    for i in left_v.iter() {
        if records.contains_key(i) {
            sum += i * records.get(&i).unwrap();
            continue;
        }
        let count = right_v.iter().filter(|n| *n == i).count();
        sum += i * count;
        records.insert(*i, count);
    }
    println!("Sum: {:?}", sum);
    
    // left_v.sort();
    // right_v.sort();
    
    // assert_eq!(left_v.len(), right_v.len());
    // let mut sum: u64 = 0;
    // for i in 0..left_v.len() {
    //     sum += left_v[i].abs_diff(right_v[i]);
    // }
    
    // println!("Sum: {:?}", sum);
}