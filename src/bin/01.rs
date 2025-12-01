advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut t: i64 = 50;
    let mut c: u64 = 0;
    for line in input.lines() {
        let (begin, end) = line.split_at_checked(1)?;
        let d = end.parse::<i64>().ok()?;
        match begin {
            "R" => {
                t += d;
            }
            "L" => {
                t -= d;
            }
            _ => (),
        }
        t = t.rem_euclid(100);
        if t == 0 {
            c += 1;
        }
    }
    Some(c)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut t: i64 = 50;
    let mut c: i64 = 0;
    for line in input.lines() {
        let (begin, end) = line.split_at_checked(1)?;
        let d = end.parse::<i64>().ok()?;
        let t_old = t;
        match begin {
            "R" => {
                t += d;
                c += t.div_euclid(100) - t_old.div_euclid(100);
            }
            "L" => {
                t -= d;
                c += (t_old - 1).div_euclid(100) - (t - 1).div_euclid(100);
            }
            _ => (),
        }
        t = t.rem_euclid(100);
    }
    Some(c)
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
        assert_eq!(result, Some(6));
    }
}
