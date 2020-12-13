use std::{
    io::{self, BufRead},
    ops::Add,
    str::FromStr,
};

use anyhow::{Context, Result};

#[derive(Copy, Clone)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = match s {
            "N" => Action::North,
            "S" => Action::South,
            "E" => Action::East,
            "W" => Action::West,
            "L" => Action::Left,
            "R" => Action::Right,
            "F" => Action::Forward,
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}

#[derive(Copy, Clone)]
struct Instruction(Action, i32);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(1);
        let action = a.parse()?;
        let argument = b.parse()?;
        let operation = Instruction(action, argument);
        Ok(operation)
    }
}

#[derive(Copy, Clone)]
struct Point(i32, i32);

impl Add<Instruction> for Point {
    type Output = Point;

    fn add(self, rhs: Instruction) -> Self::Output {
        let Point(x, y) = self;
        match rhs {
            Instruction(Action::North, arg) => Point(x, y - arg),
            Instruction(Action::South, arg) => Point(x, y + arg),
            Instruction(Action::East, arg) => Point(x + arg, y),
            Instruction(Action::West, arg) => Point(x - arg, y),
            _ => unimplemented!(),
        }
    }
}

impl Add<Instruction> for (Point, Point) {
    type Output = (Point, Point);

    fn add(self, rhs: Instruction) -> Self::Output {
        let (ship, direction) = self;
        let Point(sx, sy) = ship;
        let Point(dx, dy) = direction;
        match rhs {
            Instruction(Action::Left, arg) => (ship, direction.rotate(-arg)),
            Instruction(Action::Right, arg) => (ship, direction.rotate(arg)),
            Instruction(Action::Forward, arg) => (Point(sx + dx * arg, sy + dy * arg), direction),
            _ => unimplemented!(),
        }
    }
}

impl Point {
    fn rotate(&self, degrees: i32) -> Point {
        let Point(x, y) = *self;
        match degrees {
            0 => Point(x, y),
            90 | -270 => Point(-y, x),
            180 | -180 => Point(-x, -y),
            270 | -90 => Point(y, -x),
            _ => unreachable!(),
        }
    }
}

fn part1(input: &[Instruction]) {
    let state = (Point(0, 0), Point(1, 0));
    let (Point(x, y), _) =
        input
            .iter()
            .fold(state, |(ship, direction), &instruction| match instruction {
                Instruction(Action::Left, _)
                | Instruction(Action::Right, _)
                | Instruction(Action::Forward, _) => (ship, direction) + instruction,
                _ => (ship + instruction, direction),
            });

    let answer = x.abs() + y.abs();
    println!("Part 1: {}", answer);
}

fn part2(input: &[Instruction]) {
    let state = (Point(0, 0), Point(10, -1));
    let (Point(x, y), _) =
        input
            .iter()
            .fold(state, |(ship, waypoint), &instruction| match instruction {
                Instruction(Action::Left, _)
                | Instruction(Action::Right, _)
                | Instruction(Action::Forward, _) => (ship, waypoint) + instruction,
                _ => (ship, waypoint + instruction),
            });

    let answer = x.abs() + y.abs();
    println!("Part 2: {}", answer);
}

fn main() -> Result<()> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.context("Unable to read line").and_then(|l| l.parse()))
        .collect::<Result<Vec<Instruction>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}
