use anyhow::{Context, Result};
use itertools::Itertools;
use std::io::{self, BufRead};

const VALUE: u32 = 2020;

fn solve(size: usize, data: &[u32]) -> u32 {
    let entries = data
        .iter()
        .combinations(size)
        .find(|entries| entries.iter().cloned().sum::<u32>() == VALUE)
        .unwrap();

    entries.iter().cloned().product::<u32>()
}

fn main() -> Result<()> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.context("Failed to read line"))
        .map(|line| line.and_then(|n| n.parse().context("Failed to parse number")))
        .collect::<Result<Vec<u32>>>()?;

    println!("Part 1: {}", solve(2, &input));
    println!("Part 2: {}", solve(3, &input));

    Ok(())
}
