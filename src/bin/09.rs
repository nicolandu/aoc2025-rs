use itertools::Itertools;
use planar_convex_hull::ConvexHull;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    /* Actually faster than the naive approach! */
    let pos = input
        .lines()
        .filter_map(|l| {
            let (x, y) = l.split_once(',')?;
            Some::<(i64, i64)>((x.parse().ok()?, y.parse().ok()?))
        })
        .map(|(x, y)| [x as f64, y as f64])
        .collect_vec();
    let hull_i = pos.convex_hull();
    let pts: Vec<(i64, i64)> = hull_i
        .iter()
        .map(|i| pos.convex_hull_get(*i))
        .map(|p| (p[0] as i64, p[1] as i64))
        .collect();
    pts.iter()
        .flat_map(|p1| {
            pts.iter()
                .map(|p2| ((p2.0 - p1.0).abs() + 1) * ((p2.1 - p1.1).abs() + 1))
        })
        .max()
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
