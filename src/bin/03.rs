advent_of_code::solution!(3);
use std::collections::HashSet;
use itertools::Itertools;

fn _move(x: isize, y: isize, direction: char) -> (isize, isize) {
    match direction {
        '^' => (x, y-1),
        'v' => (x, y+1),
        '<' => (x-1, y),
        '>' => (x+1, y),
        _ => (x,y)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut vec: HashSet<(isize, isize)> = HashSet::new();
    vec.insert((0,0));
    let mut pos: (isize, isize) = (0,0);
    for char in input.chars() {
        pos = _move(pos.0, pos.1, char);
        vec.insert(pos);
    }
    Some(vec.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut vec: HashSet<(isize, isize)> = HashSet::new();
    vec.insert((0,0));
    let mut pos1: (isize, isize) = (0,0);
    let mut pos2: (isize, isize) = (0,0);
    for (char1, char2) in input.chars().tuples() {
        pos1 = _move(pos1.0, pos1.1, char1);
        pos2 = _move(pos2.0, pos2.1, char2);
        vec.insert(pos1);
        vec.insert(pos2);
    }
    Some(vec.len() as u64)
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
