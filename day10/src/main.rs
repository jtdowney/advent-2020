use std::collections::HashMap;

use anyhow::{Context, Result};

fn part1(input: &[usize]) {
    let (ones, threes) = input
        .iter()
        .scan(0, |prev, &value| {
            let diff = value - *prev;
            *prev = value;

            Some(diff)
        })
        .fold((0, 1), |(mut ones, mut threes), difference| {
            match difference {
                1 => ones += 1,
                3 => threes += 1,
                _ => {}
            }

            (ones, threes)
        });

    println!("Part 1: {}", ones * threes);
}

fn part2(input: &[usize]) {
    let goal = input.last().unwrap();
    let mut cache = HashMap::new();

    for &i in input {
        cache.insert(i, 0);

        if i <= 3 {
            cache.entry(i).and_modify(|v| *v += 1);
        }

        let end = i.min(3);
        let total = (1..=end)
            .filter_map(|n| {
                let index = i - n;
                cache.get(&index)
            })
            .sum::<u64>();
        cache.entry(i).and_modify(|v| *v += total);
    }

    let answer = cache[goal];
    println!("Part 2: {}", answer);
}

fn main() -> Result<()> {
    let mut input = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().context("unable to parse line"))
        .collect::<Result<Vec<usize>>>()?;
    input.sort_unstable();

    part1(&input);
    part2(&input);

    Ok(())
}
