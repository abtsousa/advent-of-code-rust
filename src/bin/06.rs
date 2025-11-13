use regex::Regex;
use bitmaps::Bitmap;

advent_of_code::solution!(6);

enum Command {
    TurnOn(usize, usize, usize, usize),
    TurnOff(usize, usize, usize, usize),
    Toggle(usize, usize, usize, usize)
}

fn parse_cmd(line: &str) -> Option<Command> {
    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let caps = re.captures(line)?;
    let x1 = caps.get(2)?.as_str().parse::<usize>().ok()?;
    let y1 = caps.get(3)?.as_str().parse::<usize>().ok()?;
    let x2 = caps.get(4)?.as_str().parse::<usize>().ok()?;
    let y2 = caps.get(5)?.as_str().parse::<usize>().ok()?;
    match caps.get(1)?.as_str() {
        "turn on"  => Some(Command::TurnOn(x1, y1, x2, y2)),
        "turn off" => Some(Command::TurnOff(x1, y1, x2, y2)),
        "toggle"   => Some(Command::Toggle(x1, y1, x2, y2)),
        _ => None
    }
}

fn create_bitmap(start: usize, end: usize) -> Bitmap<1000> {
    assert!(end < 1000 && start <= end);
    let mut arr = [0u128; 8];

    let start_chunk = start / 128;
    let end_chunk = end / 128;

    if start_chunk == end_chunk {
        let width = end - start + 1;
        arr[start_chunk] = ((1u128 << width) - 1) << (start % 128);
    } else {
        // fill the first partial chunk
        arr[start_chunk] = !0u128 << (start % 128);
        // fill full middle chunks
        (start_chunk + 1 .. end_chunk).for_each(|i| {
            arr[i] = !0u128;
        });
        // fill the last partial chunk
        arr[end_chunk] = (1u128 << ((end % 128) + 1)) - 1;
    }

    Bitmap::<1000>::from_value(arr)
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut lights: [Bitmap<1000>; 1000] = [Bitmap::<1000>::new(); 1000];
    for line in input.lines() {
        let cmd = parse_cmd(line)?;
        match cmd {
            Command::TurnOn(x1, y1, x2, y2) => {
                let mask = create_bitmap(x1, x2);
                (y1..=y2).for_each(|y| {
                    lights[y] |= mask
                });
            },
            Command::TurnOff(x1, y1, x2, y2) => {
                let mut mask = create_bitmap(x1, x2);
                mask.invert();
                (y1..=y2).for_each(|y| {
                    lights[y] &= mask
                });
            },
            Command::Toggle(x1, y1, x2, y2) => {
                let mask = create_bitmap(x1, x2);
                (y1..=y2).for_each(|y| {
                    lights[y] ^= mask
                });
            },
        }
    }
    let count = lights.iter().fold(0, |acc, l| acc + l.len());
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lights = [[0u8; 1000]; 1000];
    for line in input.lines() {
        let cmd = parse_cmd(line)?;
        match cmd {
            Command::TurnOn(x1, y1, x2, y2) => {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        lights[y][x] = lights[y][x].saturating_add(1)
                    }
                }
            },
            Command::TurnOff(x1, y1, x2, y2) => {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        lights[y][x] = lights[y][x].saturating_sub(1)
                    }
                }
            },
            Command::Toggle(x1, y1, x2, y2) => {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        lights[y][x] = lights[y][x].saturating_add(2)
                    }
                }
            },
        }
    }
    let count: u64 = lights.iter().flatten().map(|v| *v as u64).sum();
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
