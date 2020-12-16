use std::{collections::HashMap, iter};

const SEED: &str = "0,1,5,10,3,12,19";

fn solve(target_round: usize, seen: &HashMap<usize, usize>) -> usize {
    let mut seen = seen.clone();
    let target = target_round - seen.len();

    let mut round = seen.len();
    iter::successors(Some(0), |number| {
        let next = match seen.get(number) {
            Some(last_seen) => round - last_seen,
            None => 0,
        };

        seen.insert(*number, round);
        round += 1;

        Some(next)
    })
    .take(target)
    .last()
    .unwrap()
}

fn main() {
    let seen = SEED
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect::<HashMap<usize, usize>>();

    println!("Part 1: {}", solve(2020, &seen));
    println!("Part 2: {}", solve(30000000, &seen));
}
