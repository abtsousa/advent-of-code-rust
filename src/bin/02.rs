advent_of_code::solution!(2);

fn get_dims(line: &str) -> [u64;3] {
    let mut coords: [u64; 3] = line.split('x').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    coords.sort();
    coords
}

pub fn part_one(input: &str) -> Option<u64> {
    fn solve(coords: [u64; 3]) -> u64 {
        let [a,b,c] = coords;
        3*a*b + 2*(a*c+b*c)
    }

    Some(input.lines().map(|l| solve(get_dims(l))).sum())

}

pub fn part_two(input: &str) -> Option<u64> {
    fn solve(coords: [u64; 3]) -> u64 {
        let [a,b,c] = coords;
        2*(a+b)+a*b*c
    }

    Some(input.lines().map(|l| solve(get_dims(l))).sum())
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
