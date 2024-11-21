
pub fn run_part1(input: &str) -> usize {
    let nice_strings = input.lines().filter(|s| {
        let mut peekable = s.chars().peekable();
        let mut vowel_count = 0;
        let mut char_appear_twice = false;
        let mut prev_char = None;
        
        while let Some(c) = peekable.next() {
            if let Some(&next) = peekable.peek() {
                if (c == 'a' && next == 'b') || (c == 'c' && next == 'd') || (c == 'p' && next == 'q') || (c == 'x' && next == 'y') {
                    return false;
                }
            }
            
            if let Some(prev) = prev_char {
                if prev == c {
                    char_appear_twice = true;
                }
            }
            prev_char = Some(c);
            
            if "aeiou".contains(c) {
                vowel_count += 1;
            }
        };
        
        vowel_count >= 3 && char_appear_twice
    }).collect::<Vec<_>>().len();
    
    println!("[Part1] Number of nice strings: {}", nice_strings);

    nice_strings
}

pub fn run_part2(input: &str) -> usize {
    let nice_strings = input.lines().filter(|s| {
        let mut has_repeated_pair = false;
        let mut has_repeated_with_gap = false;
        let mut pairs = std::collections::HashMap::new();
        
        let chars: Vec<char> = s.chars().collect();
        
        for i in 0..chars.len() {
            if i < chars.len() - 1 {
                let pair = (chars[i], chars[i+1]);
                if let Some(&last_index) = pairs.get(&pair) {
                    if i - last_index > 1 {
                        has_repeated_pair = true;
                    }
                } else {
                    pairs.insert(pair, i);
                }
            }
            
            if i < chars.len() - 2 && chars[i] == chars[i + 2] {
                has_repeated_with_gap = true;
            }

            if has_repeated_pair && has_repeated_with_gap {
                return true;
            }
        }
        false
    }).count();
    
    println!("[Part2]]Number of nice strings: {}", nice_strings);

    nice_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_string_ugknbfddgicrmopn() {
        let input = "ugknbfddgicrmopn";
        assert_eq!(run_part1(input), 1);
    }

    #[test]
    fn naughty_no_double_letter() {
        let input = "jchzalrnumimnmhp";
        assert_eq!(run_part1(input), 0);
    }

    #[test]
    fn naughty_contains_xy() {
        let input = "haegwjzuvuyypxyu";
        assert_eq!(run_part1(input), 0);
    }

    #[test]
    fn naughty_contains_only_one_vowel() {
        let input = "dvszwmarrgswjxmb";
        assert_eq!(run_part1(input), 0);
    }

    #[test]
    fn nice_string_qjhvhtzxzqqjkmpb() {
        let input = "qjhvhtzxzqqjkmpb";
        assert_eq!(run_part2(input), 1);
    }

    #[test]
    fn naughty_pair_but_no_repeat_with_single_letter_betn() {
        let input = "uurcxstgmygtbstg";
        assert_eq!(run_part2(input), 0);
    }

    #[test]
    fn naughty_only_repeating_letter_no_pair() {
        let input = "ieodomkazucvgmuy";
        assert_eq!(run_part2(input), 0);
    }
}