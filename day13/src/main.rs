use anyhow::{anyhow, Result};
use num_integer::Integer;

fn part1(start_time: i64, busses: &[(i64, i64)]) {
    let (bus, end_time) = (start_time..)
        .find_map(|t| {
            busses
                .iter()
                .copied()
                .find(|&(_, b)| t.gcd(&b) != 1)
                .map(|(_, b)| (b, t))
        })
        .unwrap();

    let answer = (end_time - start_time) * bus;
    println!("Part 1: {}", answer);
}

fn part2(busses: &[(i64, i64)]) {
    let product: i64 = busses.iter().map(|(_, b)| b).product();
    let answer: i64 = busses
        .iter()
        .map(|(i, b)| (b - i, b))
        .map(|(residue, modulus)| {
            let p = product / modulus;
            let g = p.extended_gcd(modulus);
            residue * (g.x % modulus + modulus) * p
        })
        .sum::<i64>()
        % product;
    println!("Part 2: {}", answer);
}

fn main() -> Result<()> {
    let mut lines = include_str!("../input.txt").lines();
    let start_time = lines
        .next()
        .and_then(|l| l.parse().ok())
        .ok_or_else(|| anyhow!("Unable to parse timestamp"))?;
    let busses = lines
        .next()
        .map(|l| {
            l.split(',')
                .enumerate()
                .filter_map(|(i, n)| n.parse().ok().map(|n| (i as i64, n)))
                .collect::<Vec<(i64, i64)>>()
        })
        .ok_or_else(|| anyhow!("Unable to read busses"))?;

    part1(start_time, &busses);
    part2(&busses);

    Ok(())
}
