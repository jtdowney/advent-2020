use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct BoardingPass(u16);

impl FromStr for BoardingPass {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .chars()
            .map(|ch| match ch {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => unreachable!(),
            })
            .collect::<String>();
        u16::from_str_radix(&id, 2).map(BoardingPass)
    }
}

fn part1(passes: &[BoardingPass]) {
    let answer = passes.iter().max().expect("No max seat");
    println!("Part 1: {}", answer.0);
}

fn part2(passes: &[BoardingPass]) {
    let mut passes = passes.to_vec();
    passes.sort_unstable();

    let neighbors = passes
        .windows(2)
        .find(|parts| parts[1].0 - parts[0].0 == 2)
        .expect("No empty seat");
    let answer = neighbors[0].0 + 1;
    println!("Part 2: {}", answer);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read input");

    let seat_ids = input
        .lines()
        .map(|line| line.parse().expect("Invalid boarding pass"))
        .collect::<Vec<BoardingPass>>();

    part1(&seat_ids);
    part2(&seat_ids);
}
