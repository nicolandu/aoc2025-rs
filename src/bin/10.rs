use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let seqs = l.split(' ').map(|s| &s[1..s.len() - 1]).collect_vec();
                let tgt = seqs[0]
                    .chars()
                    .rev()
                    .map(|c| match c {
                        '#' => 1u32,
                        '.' => 0u32,
                        _ => unreachable!("Wrong character in target"),
                    })
                    .reduce(|acc, e| acc << 1 | e)?;
                seqs[1..seqs.len() - 1]
                    .iter()
                    .filter_map(|s| {
                        s.split(',')
                            .filter_map(|x| Some(1u32 << x.parse::<u32>().ok()?))
                            .reduce(|acc, e| acc | e)
                    })
                    .powerset()
                    .filter(|ps| ps.iter().fold(0, |acc, e| acc ^ e) == tgt)
                    .map(|ps| ps.len())
                    .min()
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
