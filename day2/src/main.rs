use anyhow::{anyhow, Context, Result};
use std::io::{self, BufRead};
use std::str::FromStr;

struct Entry {
    rule: PasswordRule,
    password: String,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(entry: &str) -> Result<Self, anyhow::Error> {
        let mut parts = entry.split(':');
        let rule = parts
            .next()
            .ok_or_else(|| anyhow!("No rule to parse"))
            .and_then(|rule| rule.parse())?;
        let password = parts
            .next()
            .ok_or_else(|| anyhow!("No password to parse"))?
            .trim()
            .to_string();

        Ok(Entry { rule, password })
    }
}

struct PasswordRule {
    minimum: usize,
    maximum: usize,
    letter: char,
}

impl FromStr for PasswordRule {
    type Err = anyhow::Error;

    fn from_str(entry: &str) -> Result<Self, anyhow::Error> {
        let mut parts = entry.split_whitespace();
        let (minimum, maximum) = parts
            .next()
            .ok_or_else(|| anyhow!("No range to parse"))
            .and_then(|range| {
                range
                    .split('-')
                    .map(|n| n.parse().context("Unable to parse range"))
                    .collect::<Result<Vec<usize>>>()
            })
            .map(|range_parts| (range_parts[0], range_parts[1]))?;

        let letter = parts
            .next()
            .and_then(|value| value.chars().next())
            .ok_or_else(|| anyhow!("No letter to parse"))?;

        Ok(PasswordRule {
            minimum,
            maximum,
            letter,
        })
    }
}

fn part1_policy(rule: &PasswordRule, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == rule.letter).count();
    count >= rule.minimum && count <= rule.maximum
}

fn part2_policy(rule: &PasswordRule, password: &str) -> bool {
    let chars = password.chars().collect::<Vec<char>>();
    let a = chars[rule.minimum - 1];
    let b = chars[rule.maximum - 1];

    (a == rule.letter) ^ (b == rule.letter)
}

fn solve<F>(input: &[Entry], policy: F) -> usize
where
    F: Fn(&PasswordRule, &str) -> bool,
{
    input
        .iter()
        .filter(|entry| policy(&entry.rule, &entry.password))
        .count()
}

fn main() -> Result<()> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.context("Failed to read line"))
        .map(|line| {
            line.and_then(|entry| entry.parse())
                .context("Failed to parse entry")
        })
        .collect::<Result<Vec<Entry>>>()?;

    println!("Part 1: {}", solve(&input, part1_policy));
    println!("Part 2: {}", solve(&input, part2_policy));

    Ok(())
}
