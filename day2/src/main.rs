use anyhow::{anyhow, Context, Result};
use regex::Regex;

type PasswordRule = (usize, usize, char);
struct Entry {
    rule: PasswordRule,
    password: String,
}

fn part1_policy((minimum, maximum, letter): PasswordRule, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == letter).count();
    count >= minimum && count <= maximum
}

fn part2_policy((first, last, letter): PasswordRule, password: &str) -> bool {
    let chars = password.chars().collect::<Vec<char>>();
    let a = chars[first - 1];
    let b = chars[last - 1];

    (a == letter) ^ (b == letter)
}

fn solve<F>(input: &[Entry], policy: F) -> usize
where
    F: Fn(PasswordRule, &str) -> bool,
{
    input
        .iter()
        .filter(|entry| policy(entry.rule, &entry.password))
        .count()
}

fn main() -> Result<()> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").context("Regex build failed")?;
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let captures = re
                .captures(&line)
                .ok_or_else(|| anyhow!("Regex match failed"))?;
            let a = captures[1].parse()?;
            let b = captures[2].parse()?;
            let letter = captures[3].chars().next().unwrap();
            let password = captures[4].to_string();
            let rule = (a, b, letter);
            let entry = Entry { rule, password };
            Ok(entry)
        })
        .collect::<Result<Vec<Entry>>>()?;

    println!("Part 1: {}", solve(&input, part1_policy));
    println!("Part 2: {}", solve(&input, part2_policy));

    Ok(())
}
