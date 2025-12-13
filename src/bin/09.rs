use geo::{Covers, LineString, Polygon, Rect, coord};
use itertools::Itertools;
use planar_convex_hull::ConvexHull;

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
    let outer = Polygon::new(
        LineString::from(pts.iter().map(|&(x, y)| (x as f64, y as f64)).collect_vec()),
        vec![],
    );
    pts.iter()
        .flat_map(|p1| {
            dbg!(p1);
            pts.iter().map(|p2| {
                let (x1, y1, x2, y2) = (p1.0, p1.1, p2.0, p2.1);
                let (x1, y1, x2, y2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));
                let inner = Rect::new(
                    coord! {x: x1 as f64, y: y1 as f64},
                    coord! {x: x2 as f64, y: y2 as f64},
                );
                if outer.covers(&inner) {
                    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
                } else {
                    0
                }
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
