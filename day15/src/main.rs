use std::{collections::HashMap, iter, result};

const SEED: &str = "0,1,5,10,3,12,19";

fn solve(length: usize, seed: &HashMap<usize, usize>) -> usize {
    let mut seed = seed.clone();
    let size = length - seed.len();
    let mut round = seed.len();

    iter::successors(Some(0), |prev| {
        let next = match seed.get(prev) {
            Some(last) => round - last,
            None => 0,
        };

        seed.insert(*prev, round);
        round += 1;

        Some(next)
    })
    .take(size)
    .last()
    .unwrap()
}

fn main() {
    let seed = SEED
        .split(',')
        .map(str::parse)
        .map(result::Result::unwrap)
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect::<HashMap<usize, usize>>();

    println!("Part 1: {}", solve(2020, &seed));
    println!("Part 2: {}", solve(30000000, &seed));
}
