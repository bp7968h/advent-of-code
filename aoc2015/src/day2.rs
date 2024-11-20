pub fn run(input: &str) -> (isize, isize) {
    let format_input: Vec<_> = input.lines().map(|l| {
        let parts: Vec<isize> = l.split('x').map(|p| p.parse::<isize>().unwrap()).collect();
        [parts[0], parts[1], parts[2]]
    }).collect();

    let mut total_area: isize = 0;
    let mut total_ribbon: isize = 0;
    
    for arr in format_input.iter() {
        let area1 = arr[0] * arr[1];
        let area2 = arr[1] * arr[2];
        let area3 = arr[2] * arr[0];
        
        let smallest_area = *[area1, area2, area3].iter().min().unwrap();
        let smallest_perimeter = *[2*(arr[0] + arr[1]), 2*(arr[1] + arr[2]), 2*(arr[2] + arr[0])].iter().min().unwrap();
        
        total_area += ((2*area1) + (2*area2) + (2*area3)) + smallest_area;
        total_ribbon += smallest_perimeter + (arr[0] * arr[1] * arr[2]);
    };
    
    println!("{}, {}", total_area, total_ribbon);
    (total_area, total_ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x3x4() {
        let input = "2x3x4";
        let (total_area, total_ribbon) = run(&input);
        assert_eq!(total_area, 58);
        assert_eq!(total_ribbon, 34);
    }

    #[test]
    fn test_1x1x10() {
        let input = "1x1x10";
        let (total_area, total_ribbon) = run(&input);
        assert_eq!(total_area, 43);
        assert_eq!(total_ribbon, 14);
    }
}