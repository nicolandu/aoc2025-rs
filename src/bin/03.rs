use itertools::Itertools;

advent_of_code::solution!(3);

fn part(input: &str, num_digits: usize) -> u64 {
    input
        .lines()
        .map(|l| {
            let c = l
                .chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u64))
                .collect_vec();
            let mut num = 0;
            let mut start_idx = 0;
            'outer: for end_idx in (0..num_digits).rev() {
                for digit in (0..=9).rev() {
                    if let Some(pos) = c[start_idx..c.len() - end_idx]
                        .iter()
                        .position(|&x| x == digit)
                    {
                        num = num * 10 + digit;
                        start_idx += pos + 1;
                        continue 'outer;
                    }
                }
            }
            num
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(part(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(part(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
