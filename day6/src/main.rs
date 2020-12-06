use std::collections::HashSet;
use std::io::{self, Read};

fn part1(answers: &[Vec<HashSet<char>>]) {
    let answers = answers
        .iter()
        .map(|group| group.iter().flatten().copied().collect())
        .collect::<Vec<HashSet<char>>>();
    let answer: usize = answers.iter().map(|group| group.len()).sum();
    println!("Part 1: {}", answer);
}

fn part2(answers: &[Vec<HashSet<char>>]) {
    let answer: usize = answers
        .iter()
        .map(|group| {
            ('a'..='z')
                .filter(|c| group.iter().all(|a| a.contains(c)))
                .count()
        })
        .sum();
    println!("Part 2: {}", answer);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read input");

    let answers = input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect::<Vec<Vec<HashSet<char>>>>();

    part1(&answers);
    part2(&answers);
}
