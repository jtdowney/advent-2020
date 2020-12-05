use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct BoardingPass(usize, usize);

impl BoardingPass {
    fn seat_id(&self) -> usize {
        let BoardingPass(r, c) = self;
        r * 8 + c
    }
}

impl FromStr for BoardingPass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ((_, rh), (_, ch)) = s
            .chars()
            .fold(((0, 127), (0, 7)), |((rl, rh), (cl, ch)), c| match c {
                'F' => {
                    let mid = (rl + rh) / 2;
                    ((rl, mid), (cl, ch))
                }
                'B' => {
                    let mid = (rl + rh) / 2;
                    ((mid, rh), (cl, ch))
                }
                'R' => {
                    let mid = (cl + ch) / 2;
                    ((rl, rh), (mid, ch))
                }
                'L' => {
                    let mid = (cl + ch) / 2;
                    ((rl, rh), (cl, mid))
                }
                _ => unreachable!(),
            });

        Ok(BoardingPass(rh, ch))
    }
}

fn part1(seat_ids: &[usize]) {
    let answer = seat_ids.iter().max().expect("No max seat");
    println!("Part 1: {}", answer);
}

fn part2(seat_ids: &[usize]) {
    let mut seat_ids = seat_ids.to_vec();
    seat_ids.sort_unstable();

    let seats = seat_ids
        .windows(2)
        .find(|parts| parts[1] - parts[0] != 1)
        .expect("No empty seat");
    let answer = seats[0] + 1;
    println!("Part 2: {}", answer);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read input");

    let seat_ids = input
        .lines()
        .map(|line| line.parse::<BoardingPass>().expect("Invalid boarding pass"))
        .map(|pass| pass.seat_id())
        .collect::<Vec<usize>>();

    part1(&seat_ids);
    part2(&seat_ids);
}
