use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

fn part_one_inner(input: &str, n: usize) -> Option<usize> {
    let boxes = input
        .lines()
        .filter_map(|l| {
            let (x, y, z) = l.splitn(3, ',').collect_tuple()?;
            Some::<(i64, i64, i64)>((x.parse().ok()?, y.parse().ok()?, z.parse().ok()?))
        })
        .collect_vec();
    let mut groups = Vec::<HashSet<_>>::new();
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b1)| {
            boxes[i + 1..].iter().map(|b2| {
                (
                    *b1,
                    *b2,
                    (b2.0 - b1.0).pow(2) + (b2.1 - b1.1).pow(2) + (b2.2 - b1.2).pow(2),
                )
            })
        })
        .sorted_by_key(|p| p.2)
        .take(n)
        .for_each(|p| {
            let (b1, b2) = (p.0, p.1);
            let b1_idx = groups.iter().position(|g| g.contains(&b1));
            let b2_idx = groups.iter().position(|g| g.contains(&b2));
            match (b1_idx, b2_idx) {
                (Some(b1_idx), Some(b2_idx)) if b1_idx != b2_idx => {
                    // Merge sets while preserving indices
                    let (left, right) = (b1_idx.min(b2_idx), b1_idx.max(b2_idx));
                    let hs = groups.swap_remove(right);
                    groups[left].extend(hs);
                }
                (Some(b1_idx), None) => {
                    groups[b1_idx].insert(b2);
                }
                (None, Some(b2_idx)) => {
                    groups[b2_idx].insert(b1);
                }
                (None, None) => {
                    groups.push(HashSet::from([b1, b2]));
                }
                _ => (),
            }
        });
    Some(
        groups
            .iter()
            .sorted_by_key(|g| g.len())
            .rev()
            .take(3)
            .map(|g| g.len())
            .product(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 1000)
}

pub fn part_two(input: &str) -> Option<i64> {
    let boxes = input
        .lines()
        .filter_map(|l| {
            let (x, y, z) = l.splitn(3, ',').collect_tuple()?;
            Some::<(i64, i64, i64)>((x.parse().ok()?, y.parse().ok()?, z.parse().ok()?))
        })
        .collect_vec();
    let mut groups = boxes.iter().map(|b| HashSet::from([b])).collect_vec();
    for p in boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b1)| {
            boxes[i + 1..].iter().map(|b2| {
                (
                    *b1,
                    *b2,
                    (b2.0 - b1.0).pow(2) + (b2.1 - b1.1).pow(2) + (b2.2 - b1.2).pow(2),
                )
            })
        })
        .sorted_by_key(|p| p.2)
    {
        let (b1, b2) = (p.0, p.1);
        let b1_idx = groups.iter().position(|g| g.contains(&b1));
        let b2_idx = groups.iter().position(|g| g.contains(&b2));
        match (b1_idx, b2_idx) {
            (Some(b1_idx), Some(b2_idx)) if b1_idx != b2_idx => {
                // Merge sets while preserving indices
                let (left, right) = (b1_idx.min(b2_idx), b1_idx.max(b2_idx));
                let hs = groups.swap_remove(right);
                groups[left].extend(hs);
            }
            (Some(_), None) | (None, Some(_)) | (None, None) => unreachable!("Sanity check"),
            _ => (),
        }
        if groups.len() == 1 {
            return Some(b1.0 * b2.0);
        }
    }
    unreachable!("Error somewhere");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
