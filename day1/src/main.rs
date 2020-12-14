use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

const SEARCH_VALUE: u32 = 2020;

fn solve(size: usize, input: &[u32]) -> Result<u32> {
    let entries = input
        .iter()
        .combinations(size)
        .find(|entries| entries.iter().copied().sum::<u32>() == SEARCH_VALUE)
        .ok_or_else(|| anyhow!("Unable to find solution"))?;

    let solution = entries.iter().copied().product();
    Ok(solution)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().context("Failed to parse number"))
        .collect::<Result<Vec<u32>>>()?;

    println!("Part 1: {}", solve(2, &input)?);
    println!("Part 2: {}", solve(3, &input)?);

    Ok(())
}
