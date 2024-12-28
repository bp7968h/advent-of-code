use std::collections::HashMap;

pub fn run(input: &str) {
    let mut left_v: Vec<usize> = vec![];
    let mut right_v: Vec<usize> = vec![];
    
    parse_input(input, &mut left_v, &mut right_v);
    assert_eq!(left_v.len(), right_v.len());

    let part_one_sum = part_one(&left_v, &right_v);
    println!("Part one puzzle answer: {part_one_sum}");

    let part_two_sum = part_two(&left_v, &right_v);
    println!("Part two puzzle answer: {part_two_sum}");

}

fn parse_input(input: &str, left_v: &mut Vec<usize>, right_v: &mut Vec<usize>) {
    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        
        let (Some(left), Some(right)) = (line_iter.next(), line_iter.next()) else {
            panic!("Wrong format");
        };
        
        left_v.push(left.parse::<usize>().unwrap());
        right_v.push(right.parse::<usize>().unwrap());
    }
} 

fn part_one(left_nums: &[usize], right_nums: &[usize]) -> usize {
    let mut left_sorted: Vec<usize> = left_nums.to_owned();
    let mut right_sorted: Vec<usize> = right_nums.to_owned();
    left_sorted.sort();
    right_sorted.sort();

    let mut sum: usize = 0;
    for i in 0..left_sorted.len() {
        sum += left_sorted[i].abs_diff(right_sorted[i]);
    }

    sum
}

fn part_two(left_nums: &[usize], right_nums: &[usize]) -> usize {
    let mut records: HashMap<usize, usize> = HashMap::new();
    let mut sum: usize = 0;
    
    for i in left_nums.iter() {
        if records.contains_key(i) {
            sum += i * records.get(&i).unwrap();
            continue;
        }
        let count = right_nums.iter().filter(|n| *n == i).count();
        sum += i * count;
        records.insert(*i, count);
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_example() {
        let input: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        let mut left_input: Vec<usize> = vec![];
        let mut right_input: Vec<usize> = vec![];

        parse_input(input, &mut left_input, &mut right_input);
        assert_eq!(left_input.len(), right_input.len());

        let part_one_sum = part_one(&left_input, &right_input);
        assert_eq!(part_one_sum, 11);

        let part_two_sum = part_two(&left_input, &right_input);
        assert_eq!(part_two_sum, 31);
    }
}