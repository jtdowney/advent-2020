use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;
use std::ops::Index;

type Point = (usize, usize);
const ORIGIN: Point = (0, 0);

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Tree,
    Open,
}

struct Grid {
    pattern: HashMap<Point, Square>,
    pattern_width: usize,
    pattern_height: usize,
}

impl Grid {
    fn new(pattern: HashMap<Point, Square>) -> Self {
        let pattern_width = pattern.keys().map(|&(x, _)| x).max().unwrap();
        let pattern_height = pattern.keys().map(|&(_, y)| y).max().unwrap();
        Self {
            pattern,
            pattern_width,
            pattern_height,
        }
    }
}

impl Index<Point> for Grid {
    type Output = Square;

    fn index(&self, (x, y): Point) -> &Self::Output {
        let x = x % (self.pattern_width + 1);
        &self.pattern[&(x, y)]
    }
}

fn path(dx: usize, dy: usize) -> impl Iterator<Item = Point> {
    iter::successors(Some(ORIGIN), move |(x, y)| Some((x + dx, y + dy)))
}

fn part1(grid: &Grid) {
    let count = path(3, 1)
        .take_while(|&(_, y)| y <= grid.pattern_height)
        .filter(|&p| grid[p] == Square::Tree)
        .count();

    println!("Part 1: {}", count);
}

fn part2(grid: &Grid) {
    let paths = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = paths
        .iter()
        .map(|&(dx, dy)| {
            path(dx, dy)
                .take_while(|&(_, y)| y <= grid.pattern_height)
                .filter(|&p| grid[p] == Square::Tree)
                .count()
        })
        .product();

    println!("Part 2: {}", product);
}

fn main() {
    let pattern = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let row = line.expect("Failed to read line");
            row.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    let square = match ch {
                        '#' => Square::Tree,
                        '.' => Square::Open,
                        _ => unreachable!(),
                    };
                    ((x, y), square)
                })
                .collect::<HashMap<Point, Square>>()
        })
        .collect::<HashMap<Point, Square>>();

    let grid = Grid::new(pattern);

    part1(&grid);
    part2(&grid);
}
