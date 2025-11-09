use itertools::Itertools;

advent_of_code::solution!(5);

fn validate_duplicate_chars(input: &str) -> bool {
    let mut lastchar: char = '\n';
    for char in input.chars() {
        if char == lastchar {
            return true;
        }
        lastchar = char;
    }
    false
}

fn validate_vowels(input: &str) -> bool {
    let mut count: usize = 0;
    let counts = input.chars().counts();
    for vowel in ['a', 'e', 'i', 'o', 'u'] {
        if let Some(n) = counts.get(&vowel) {
            count += n;
        }
    }
    count > 2
}

fn validate_invalid_chars(input: &str) -> bool {
    for sequence in ["ab", "cd", "pq", "xy"] {
        if input.contains(sequence) {
            return false
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count: u64 = 0;
    for line in input.lines() {
        if validate_duplicate_chars(line) &&
           validate_invalid_chars(line) &&
           validate_vowels(line) {
               count += 1;
        }
    }
    Some(count)
}

fn validate_two_letters_repeat_pattern_no_overlap(input: &str) -> bool {
    // Sliding window approach
    // we get two letters and check for repeated pattern afterwards
    // abcdef
    // ii....
    // ..jj..
    for i in 0..input.len()-3 {
        for j in i+2..input.len()-1 {
            if input[i..=i+1] == input[j..=j+1] {
                return true
            }
        }
    }
    false
}

fn validate_duplicate_chars_with_gap(input: &str) -> bool {
    let mut chars = input.trim().chars();
    let mut n_minus2: char = chars.next().unwrap();
    let mut n_minus1: char = chars.next().unwrap();
    for char in chars {
        if char == n_minus2 {
            return true;
        }
        n_minus2 = n_minus1;
        n_minus1 = char;
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count: u64 = 0;
    for line in input.lines() {
        if validate_duplicate_chars_with_gap(line) &&
           validate_two_letters_repeat_pattern_no_overlap(line) {
               count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
