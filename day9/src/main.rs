use std::io::{self, BufRead};

use anyhow::{Context, Result};
use itertools::Itertools;

const PREAMBLE_SIZE: usize = 25;

fn part1(input: &[u64]) -> u64 {
    let answer = input
        .windows(PREAMBLE_SIZE + 1)
        .find_map(|window| {
            let sum = window[PREAMBLE_SIZE];
            let found = window[0..PREAMBLE_SIZE]
                .iter()
                .combinations(2)
                .any(|numbers| numbers.iter().copied().sum::<u64>() == sum);
            if found {
                None
            } else {
                Some(sum)
            }
        })
        .expect("answer");

    println!("Part 1: {}", answer);

    answer
}

fn part2(input: &[u64], sum: u64) {
    let answer = (2..input.len())
        .map(|n| input.windows(n))
        .find_map(|mut windows| windows.find(|window| window.iter().copied().sum::<u64>() == sum))
        .expect("answer");

    let (min, max) = answer.iter().copied().minmax().into_option().unwrap();
    let value = min + max;
    println!("Part 2: {}", value);
}

fn main() -> Result<()> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.context("unable to read line")
                .and_then(|l| l.parse().context("unable to parse line"))
        })
        .collect::<Result<Vec<u64>>>()?;

    let part1 = part1(&input);
    part2(&input, part1);

    Ok(())
}
