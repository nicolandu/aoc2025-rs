use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let seqs = l.split(' ').map(|s| &s[1..s.len() - 1]).collect_vec();
                let tgt = seqs
                    .first()?
                    .chars()
                    .rev()
                    .map(|c| match c {
                        '#' => 1u32,
                        '.' => 0u32,
                        _ => unreachable!("Wrong character in target"),
                    })
                    .reduce(|acc, e| acc << 1 | e)?;
                seqs[1..seqs.len() - 1]
                    .iter()
                    .filter_map(|s| {
                        s.split(',')
                            .filter_map(|x| Some(1u32 << x.parse::<u32>().ok()?))
                            .reduce(|acc, e| acc | e)
                    })
                    .powerset()
                    .filter(|ps| ps.iter().fold(0, |acc, e| acc ^ e) == tgt)
                    .map(|ps| ps.len())
                    .min()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let seqs = l.split(' ').map(|s| &s[1..s.len() - 1]).collect_vec();
                let switches = seqs[1..seqs.len() - 1]
                    .iter()
                    .map(|s| {
                        s.split(',')
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect_vec()
                    })
                    .collect_vec();
                let total = seqs
                    .last()?
                    .split(',')
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect_vec();
                let max_total = *total.iter().max()?;

                let mut prob = variables!();
                let vars = switches
                    .iter()
                    .map(|_val| prob.add(variable().integer().min(0).max(max_total)))
                    .collect_vec();

                let mut min_expr = Expression::from(0);
                for var in vars.iter() {
                    min_expr += var;
                }
                let mut prob = prob.minimise(min_expr).using(default_solver);

                let mut exprs = vec![Expression::from(0); total.len()];
                for (i, switch) in switches.iter().enumerate() {
                    for &v in switch {
                        *exprs.get_mut(v)? += vars.get(i)?;
                    }
                }
                for (&total, expr) in total.iter().zip(exprs.into_iter()) {
                    prob = prob.with(constraint!(expr == total));
                }
                let prob = prob
                    .solve()
                    .expect("Hi there, there's been a problem solving stuff...");
                Some(vars.iter().map(|&v| prob.value(v) as u32).sum::<u32>())
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
