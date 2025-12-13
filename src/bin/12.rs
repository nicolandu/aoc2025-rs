use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let blocks = input.split("\n\n").collect_vec();
    Some(
        blocks
            .last()?
            .lines()
            .map(|l| {
                let (label, rest) = l.split_once(": ")?;
                let (x, y) = label.split_once('x')?;
                let (x, y) = (x.parse::<u32>().ok()?, y.parse::<u32>().ok()?);
                let size = x * y;
                Some(
                    rest.split(' ')
                        .filter_map(|n| n.parse::<u32>().ok())
                        .sum::<u32>()
                        * 9
                        <= size,
                )
            })
            .filter(|opt| opt.is_some_and(|b| b))
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}
