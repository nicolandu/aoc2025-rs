use advent_of_code::{Grid, NEIGHBOURS_ALL_VECTORS};

advent_of_code::solution!(4);

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Roll,
    Empty,
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::parse(input, |(_y, _x), c| match c {
        '@' => Tile::Roll,
        _ => Tile::Empty,
    });
    Some(
        grid.iter_tiles()
            .filter(|&(_pos, t)| *t == Tile::Roll)
            .filter(|((y, x), _t)| {
                NEIGHBOURS_ALL_VECTORS
                    .iter()
                    .filter(|(dy, dx)| grid.get((y + dy, x + dx)) == Some(&Tile::Roll))
                    .count()
                    < 4
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::parse(input, |(_y, _x), c| match c {
        '@' => Tile::Roll,
        _ => Tile::Empty,
    });
    let mut cnt = 0;
    loop {
        let mut delta = 0;
        grid = grid.map_collect(|(y, x), t| {
            if *t == Tile::Empty {
                return Tile::Empty;
            }
            if NEIGHBOURS_ALL_VECTORS
                .iter()
                .filter(|(dy, dx)| grid.get((y + dy, x + dx)) == Some(&Tile::Roll))
                .count()
                < 4
            {
                cnt += 1;
                delta += 1;
                Tile::Empty
            } else {
                Tile::Roll
            }
        });
        if delta == 0 {
            return Some(cnt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
