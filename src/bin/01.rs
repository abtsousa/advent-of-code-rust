advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let count: i64 = input.chars().map(|c| if c == '(' {1} else {-1}).sum();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sol = input.chars().map(|c| if c == '(' {1} else {-1})
        .try_fold((0, 0), |(i, acc), x| if acc == -1 {Err(i)} else {Ok((i+1, acc+x))})
        .unwrap_err(); // we use Err to short-circuit and exit the fold early
    Some(sol as u64)
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
