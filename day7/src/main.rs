use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::io::Read;

const SEARCH: &str = "shiny gold";

#[derive(Copy, Clone)]
struct Rule<'a> {
    count: usize,
    name: &'a str,
}

fn part1(rules: &HashMap<&str, Vec<Rule>>) {
    let answer = rules
        .keys()
        .filter(|&name| *name != SEARCH)
        .filter(|&name| {
            let mut search = VecDeque::new();
            search.push_back(*name);

            while let Some(entry) = search.pop_front() {
                let children = rules[entry].iter().map(|r| r.name).collect::<Vec<&str>>();
                if children.contains(&SEARCH) {
                    return true;
                } else {
                    search.extend(children);
                }
            }

            false
        })
        .count();

    println!("Part 1: {}", answer);
}

fn part2(rules: &HashMap<&str, Vec<Rule>>) {
    let mut count = 0;
    let mut search = VecDeque::new();
    search.push_back((1, SEARCH));

    while let Some((n, entry)) = search.pop_front() {
        for rule in &rules[entry] {
            let weight = n * rule.count;
            count += weight;
            search.push_back((weight, rule.name));
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read input");

    let name_re = Regex::new(r"^(.+) bags contain").expect("Unable to compile regex");
    let rule_re = Regex::new(r"(\d+) (.+?) bags?").expect("Unable to compile regex");
    let rules = input
        .lines()
        .map(|line| {
            let name_match = name_re.captures(line).expect("name match");
            let name = name_match.get(1).map(|m| m.as_str()).unwrap();
            let rules = rule_re
                .captures_iter(line)
                .map(|rule_capture| {
                    let count = rule_capture
                        .get(1)
                        .and_then(|m| m.as_str().parse().ok())
                        .unwrap();
                    let name = rule_capture.get(2).map(|m| m.as_str()).unwrap();
                    Rule { count, name }
                })
                .collect();
            (name, rules)
        })
        .collect::<HashMap<&str, Vec<Rule>>>();

    part1(&rules);
    part2(&rules);
}
