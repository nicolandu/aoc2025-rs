use itertools::Itertools;

advent_of_code::solution!(2);

fn first_repeat_starting(x: u64) -> u64 {
    let log = x.ilog10();
    match log % 2 {
        0 => {
            // Odd number of digits
            10u64.pow(log / 2)
        }
        1 => {
            // Even number of digits
            let mask = 10u64.pow(log.div_ceil(2));
            let (hi, lo) = (x / mask, x % mask);
            if lo > hi { hi + 1 } else { hi }
        }
        _ => unreachable!(),
    }
}

fn last_repeat_ending(x: u64) -> Option<u64> {
    let log = x.ilog10();
    let ret = match log % 2 {
        0 => {
            // Odd number of digits
            10u64.pow(log / 2) - 1
        }
        1 => {
            // Even number of digits
            let mask = 10u64.pow(log.div_ceil(2));
            let (hi, lo) = (x / mask, x % mask);
            if lo >= hi { hi } else { hi - 1 }
        }
        _ => unreachable!(),
    };
    (ret != 0).then_some(ret)
}

pub fn part_one(input: &str) -> Option<u64> {
    input.trim_end().split(',').try_fold(0, |acc, s| {
        let (a, b) = s.split_once('-')?;
        let (a, b) = (a.parse().ok()?, b.parse().ok()?);
        let sum: u64 = (first_repeat_starting(a)..=last_repeat_ending(b)?)
            .map(|x| x * (1 + 10u64.pow(x.ilog10() + 1)))
            .sum();
        Some(acc + sum)
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    input.trim_end().split(',').try_fold(0, |acc, s| {
        let (a, b) = s.split_once('-')?;
        let (a, b): (u64, u64) = (a.parse().ok()?, b.parse().ok()?);
        let sum: u64 = (a..=b)
            .filter(|&x| {
                let imo = u64::is_multiple_of;
                match x.ilog10() + 1 {
                    1 => false,
                    2 => imo(x, 11),
                    3 => imo(x, 111),
                    4 => imo(x, 101),
                    5 => imo(x, 11111),
                    6 => imo(x, 1001) || imo(x, 10101),
                    7 => imo(x, 1111111),
                    8 => imo(x, 10001),
                    9 => imo(x, 1001001),
                    10 => imo(x, 101010101) || imo(x, 100001),
                    11 => imo(x, 11111111111),
                    12 => imo(x, 1000001),
                    13 => imo(x, 1111111111111),
                    14 => imo(x, 1010101010101) || imo(x, 10000001),
                    15 => imo(x, 1001001001001) || imo(x, 10000100001),
                    16 => imo(x, 100000001),
                    17 => imo(x, 11111111111111111),
                    18 => imo(x, 1000001000001) || imo(x, 1000000001),
                    19 => imo(x, 1111111111111111111),
                    _ => unreachable!(),
                }
            })
            .sum();
        Some(acc + sum)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
