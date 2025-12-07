advent_of_code::solution!(7);
use advent_of_code::Grid;

pub fn part_one(input: &str) -> Option<u64> {
    let mut start_x = None;
    let grid = Grid::parse(input, |(_y, x), c| {
        if c == 'S' {
            start_x = Some(x);
        }
        c == '^'
    });
    let start_x = start_x?;
    let mut cur_row = vec![0u64; grid.width.try_into().ok()?];
    cur_row[usize::try_from(start_x).ok()?] = 1;

    let mut cnt = 0;
    for row in 1..grid.height {
        let mut new_row = vec![0; grid.width.try_into().ok()?];
        for col in 0..grid.width {
            let ucol = usize::try_from(col).ok()?;
            if col > 0 && col < grid.width - 1 && grid[(row, col)] && cur_row[ucol] > 0 {
                // splitter
                cnt += 1;
                new_row[ucol + 1] += cur_row[ucol];
                new_row[ucol - 1] += cur_row[ucol];
            } else {
                new_row[ucol] += cur_row[ucol];
            }
        }
        cur_row = new_row;
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start_x = None;
    let grid = Grid::parse(input, |(_y, x), c| {
        if c == 'S' {
            start_x = Some(x);
        }
        c == '^'
    });
    let start_x = start_x?;
    let mut cur_row = vec![0u64; grid.width.try_into().ok()?];
    cur_row[usize::try_from(start_x).ok()?] = 1;

    for row in 1..grid.height {
        let mut new_row = vec![0; grid.width.try_into().ok()?];
        for col in 0..grid.width {
            let ucol = usize::try_from(col).ok()?;
            if col > 0 && col < grid.width - 1 && grid[(row, col)] && cur_row[ucol] > 0 {
                // splitter
                new_row[ucol + 1] += cur_row[ucol];
                new_row[ucol - 1] += cur_row[ucol];
            } else {
                new_row[ucol] += cur_row[ucol];
            }
        }
        cur_row = new_row;
    }

    Some(cur_row.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
