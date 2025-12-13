use std::{collections::HashMap, iter::once};

use itertools::Itertools;
use planar_convex_hull::ConvexHull;
use range_union_find::{OverlapType, RangeUnionFind};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    /* Actually faster than the naive approach! */
    let pos = input
        .lines()
        .filter_map(|l| {
            let (x, y) = l.split_once(',')?;
            Some::<(i32, i32)>((x.parse().ok()?, y.parse().ok()?))
        })
        .map(|(x, y)| [x as f64, y as f64])
        .collect_vec();
    let hull_i = pos.convex_hull();
    let pts: Vec<(i32, i32)> = hull_i
        .iter()
        .map(|i| pos.convex_hull_get(*i))
        .map(|p| (p[0] as i32, p[1] as i32))
        .collect();
    pts.iter()
        .flat_map(|p1| {
            pts.iter()
                .map(|p2| ((p2.0 - p1.0).abs() + 1) * ((p2.1 - p1.1).abs() + 1))
        })
        .max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let pts = input
        .lines()
        .filter_map(|l| {
            let (x, y) = l.split_once(',')?;
            Some::<(i32, i32)>((x.parse().ok()?, y.parse().ok()?))
        })
        .collect_vec();
    let mut horiz = HashMap::<_, RangeUnionFind<_>>::new();
    let mut vert = HashMap::<_, RangeUnionFind<_>>::new();
    for (&(x1, y1), &(x2, y2)) in pts
        .iter()
        .tuple_windows()
        .chain(once((pts.last()?, pts.first()?)))
    {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                vert.entry(y)
                    .or_insert_with(RangeUnionFind::new)
                    .insert_range(&(x1..=x1))
                    .ok()?
            }
        } else {
            for x in x1.min(x2)..=x1.max(x2) {
                horiz
                    .entry(x)
                    .or_insert_with(RangeUnionFind::new)
                    .insert_range(&(y1..=y1))
                    .ok()?
            }
        }
    }
    pts.iter()
        .flat_map(|p1| {
            pts.iter().map(|p2| {
                let (x1, y1, x2, y2) = (p1.0, p1.1, p2.0, p2.1);
                let (x1p, y1p, x2p, y2p) = (
                    x1.min(x2) + 1,
                    y1.min(y2) + 1,
                    x1.max(x2) - 1,
                    y1.max(y2) - 1,
                );
                // Let's not care about 1-wide inner rectangles
                if x2p < x1p || y2p < y1p {
                    return 0;
                }
                if vert
                    .get(&y1p)
                    .and_then(|r| Some(r.has_range(&(x1p..=x2p)).ok()? != OverlapType::Disjoint))
                    .unwrap_or(false)
                    || vert
                        .get(&y2p)
                        .and_then(|r| {
                            Some(r.has_range(&(x1p..=x2p)).ok()? != OverlapType::Disjoint)
                        })
                        .unwrap_or(false)
                    || horiz
                        .get(&x1p)
                        .and_then(|r| {
                            Some(r.has_range(&(y1p..=y2p)).ok()? != OverlapType::Disjoint)
                        })
                        .unwrap_or(false)
                    || horiz
                        .get(&x2p)
                        .and_then(|r| {
                            Some(r.has_range(&(y1p..=y2p)).ok()? != OverlapType::Disjoint)
                        })
                        .unwrap_or(false)
                {
                    return 0;
                }
                ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
            })
        })
        .max()
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
        assert_eq!(result, Some(24));
    }
}
