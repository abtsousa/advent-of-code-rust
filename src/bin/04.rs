advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut i: usize = 0;
    loop {
        let i_str = format!("{}{}", input.trim(), i);
        let hash = format!("{:032x}", md5::compute(i_str));
        if &hash[..5] == "00000" {
            break
        }
        i +=1;
    };
    Some(i as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut i: usize = 0;
    loop {
        let i_str = format!("{}{}", input.trim(), i);
        let hash = format!("{:032x}", md5::compute(i_str));
        if &hash[..6] == "000000" {
            break
        }
        i +=1;
    };
    Some(i as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
