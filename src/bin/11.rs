use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(11);

struct Rec {
    adj: Vec<u32>,
    ways: u64,
}

const START1: u32 = hash("you");
const END1: u32 = hash("out");

const START2: u32 = hash("svr");
const VIA2_1: u32 = hash("dac");
const VIA2_2: u32 = hash("fft");
const END2: u32 = hash("out");

const fn hash(label: &str) -> u32 {
    let bytes = label.as_bytes();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0])
}

fn make_map(input: &str, global_end: u32) -> Option<HashMap<u32, Rec>> {
    let mut map = HashMap::new();
    for l in input.lines() {
        let (label, rest) = l.split_once(": ")?;
        map.insert(
            hash(label),
            Rec {
                ways: 0,
                adj: rest.split(' ').map(hash).collect_vec(),
            },
        );
    }
    map.insert(
        global_end,
        Rec {
            adj: Vec::new(),
            ways: 0,
        },
    );
    Some(map)
}

fn ways_between(map: &mut HashMap<u32, Rec>, start: u32, end: u32) -> Option<u64> {
    for v in map.values_mut() {
        v.ways = 0;
    }
    let mut queue = VecDeque::from([(start, 1u64)]);
    while let Some((pos, delta)) = queue.pop_front() {
        let cur = map.get_mut(&pos)?;
        cur.ways += delta;
        for &neighbour in &cur.adj.clone() {
            if let Some(entry) = queue.iter_mut().find(|p| p.0 == neighbour) {
                entry.1 += delta;
            } else {
                queue.push_back((neighbour, delta));
            };
        }
    }
    Some(map.get(&end)?.ways)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = make_map(input, END1)?;
    ways_between(&mut map, START1, END1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = make_map(input, END2)?;
    Some(
        ways_between(&mut map, START2, VIA2_1)?
            * ways_between(&mut map, VIA2_1, VIA2_2)?
            * ways_between(&mut map, VIA2_2, END2)?
            + ways_between(&mut map, START2, VIA2_2)?
                * ways_between(&mut map, VIA2_2, VIA2_1)?
                * ways_between(&mut map, VIA2_1, END2)?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
