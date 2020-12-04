use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

lazy_static! {
    static ref REQUIRED_KEYS: HashSet<&'static str> = {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .copied()
            .collect::<HashSet<&'static str>>()
    };
    static ref EYE_COLORS: HashSet<&'static str> = {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .copied()
            .collect::<HashSet<&'static str>>()
    };
}

fn part1_validator(entry: &HashMap<&str, &str>) -> bool {
    let keys = entry.keys().copied().collect::<HashSet<&str>>();
    REQUIRED_KEYS.is_subset(&keys)
}

fn part2_validator(entry: &HashMap<&str, &str>) -> bool {
    REQUIRED_KEYS.iter().all(|&key| {
        entry
            .get(key)
            .map(|value| match key {
                "byr" => value
                    .parse::<u16>()
                    .map(|n| n >= 1920 && n <= 2002)
                    .unwrap_or_default(),
                "iyr" => value
                    .parse::<u16>()
                    .map(|n| n >= 2010 && n <= 2020)
                    .unwrap_or_default(),
                "eyr" => value
                    .parse::<u16>()
                    .map(|n| n >= 2020 && n <= 2030)
                    .unwrap_or_default(),
                "hgt" => {
                    let position = value.bytes().position(|b| !b.is_ascii_digit());
                    if let Some(index) = position {
                        let (amount, unit) = value.split_at(index);
                        let amount = amount.parse::<u16>();
                        match (unit, amount) {
                            ("cm", Ok(v)) if v >= 150 && v <= 193 => true,
                            ("in", Ok(v)) if v >= 59 && v <= 76 => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                "hcl" => {
                    let (start, value) = value.split_at(1);
                    if start == "#" && value.len() == 6 {
                        u32::from_str_radix(value, 16).is_ok()
                    } else {
                        false
                    }
                }
                "ecl" => EYE_COLORS.contains(value),
                "pid" => {
                    if value.len() == 9 {
                        value.parse::<u32>().is_ok()
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            })
            .unwrap_or_default()
    })
}

fn solve<F>(entries: &[HashMap<&str, &str>], validator: F) -> usize
where
    F: Fn(&HashMap<&str, &str>) -> bool,
{
    entries.iter().filter(|entry| validator(entry)).count()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read input");
    let entries = input
        .split("\n\n")
        .map(|row| {
            row.trim()
                .split_whitespace()
                .fold(HashMap::new(), |mut acc, part| {
                    let mut values = part.split(':');
                    let key = values.next().expect("key");
                    let value = values.next().expect("value");
                    acc.insert(key, value);
                    acc
                })
        })
        .collect::<Vec<HashMap<&str, &str>>>();

    println!("Part 1: {}", solve(&entries, part1_validator));
    println!("Part 2: {}", solve(&entries, part2_validator));
}
