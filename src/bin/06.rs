use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();
    let mut sum = 0;
    let last = lines.last()?;
    for i in 0..lines.first()?.len() {
        let parsed = lines.iter().filter_map(|l| l[i].parse::<u64>().ok());
        match last[i] {
            "+" => sum += parsed.sum::<u64>(),
            "*" => sum += parsed.product::<u64>(),
            _ => panic!("Wrong operator"),
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut last = lines.last()?.iter().enumerate().collect_vec();
    last.reverse();
    let mut ops = Vec::new();
    let last = last
        .split_inclusive(|(_i, c)| match c {
            '+' => {
                ops.push(Op::Add);
                true
            }
            '*' => {
                ops.push(Op::Mul);
                true
            }
            _ => false,
        })
        .map(|r| r.iter().map(|(i, _c)| i).collect_vec())
        .collect_vec();

    let mut sum = 0;
    for (idxs, op) in last.iter().zip(&ops) {
        let parsed = idxs.iter().filter_map(|&&c| {
            (0..lines.len() - 1)
                .map(|r| lines[r][c])
                .filter(|c| !c.is_whitespace())
                .join("")
                .parse::<u64>()
                .ok()
        });
        match op {
            Op::Add => sum += parsed.sum::<u64>(),
            Op::Mul => sum += parsed.product::<u64>(),
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
