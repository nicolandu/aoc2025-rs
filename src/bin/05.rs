use range_union_find::RangeUnionFind;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, ids) = input.split_once("\n\n")?;

    let mut set = RangeUnionFind::<u64>::new();

    dbg!(
        ranges
            .lines()
            .filter_map(|l| {
                let (_a, b) = l.split_once('-')?;
                b.parse::<u64>().ok()
            })
            .max()
    );

    for r in ranges.lines().filter_map(|l| {
        let (a, b) = l.split_once('-')?;
        let (a, b) = (a.parse::<u64>().ok()?, b.parse::<u64>().ok()?);
        Some(a..=b)
    }) {
        set.insert_range(&r).ok()?;
    }

    Some(
        ids.lines()
            .filter_map(|l| l.parse::<u64>().ok())
            .filter(|x| set.has_element(x))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, _ids) = input.split_once("\n\n")?;

    let mut set = RangeUnionFind::<u64>::new();

    for r in ranges.lines().filter_map(|l| {
        let (a, b) = l.split_once('-')?;
        let (a, b) = (a.parse::<u64>().ok()?, b.parse::<u64>().ok()?);
        Some(a..=b)
    }) {
        set.insert_range(&r).ok()?;
    }
    Some(
        set.to_collection::<Vec<_>>()
            .into_iter()
            .map(|x| x.count())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
