use std::{
    collections::HashMap,
    io::{self, BufRead},
    iter,
};

type Point = (isize, isize);

const NEIGHBORS: [Point; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, PartialEq)]
struct SeatingArea {
    seats: HashMap<Point, Seat>,
}

impl SeatingArea {
    fn count_occupied_immediate_neighbors(&self, (x, y): Point) -> usize {
        NEIGHBORS
            .iter()
            .map(|&(dx, dy)| (x + dx, y + dy))
            .filter(|n| {
                self.seats
                    .get(&n)
                    .map(|&seat| seat == Seat::Occupied)
                    .unwrap_or_default()
            })
            .count()
    }

    fn count_occupied_line_of_sight_neighbors(&self, (x, y): Point) -> usize {
        let width = self.seats.keys().map(|&(x, _)| x).max().unwrap();
        let height = self.seats.keys().map(|&(_, y)| y).max().unwrap();

        NEIGHBORS
            .iter()
            .filter_map(|&(dx, dy)| {
                let neighbor = (x + dx, y + dy);
                iter::successors(Some(neighbor), |(nx, ny)| Some((nx + dx, ny + dy)))
                    .take_while(|&(nx, ny)| nx >= 0 && nx <= width && ny >= 0 && ny <= height)
                    .find_map(|n| self.seats.get(&n))
            })
            .filter(|&seat| *seat == Seat::Occupied)
            .count()
    }

    fn step_immedate(&self) -> Self {
        let seats = self
            .seats
            .iter()
            .map(|(&point, &seat)| {
                let count = self.count_occupied_immediate_neighbors(point);
                let next = match (seat, count) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, count) if count >= 4 => Seat::Empty,
                    (s, _) => s,
                };
                (point, next)
            })
            .collect();

        SeatingArea { seats }
    }

    fn step_line_of_sight(&self) -> Self {
        let seats = self
            .seats
            .iter()
            .map(|(&point, &seat)| {
                let count = self.count_occupied_line_of_sight_neighbors(point);
                let next = match (seat, count) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, count) if count >= 5 => Seat::Empty,
                    (s, _) => s,
                };
                (point, next)
            })
            .collect();

        SeatingArea { seats }
    }
}

fn solve<F>(area: &SeatingArea, step: F) -> usize
where
    F: Fn(&SeatingArea) -> Option<SeatingArea>,
{
    let stable = iter::successors(Some(area.clone()), step)
        .try_fold(None, |prev: Option<SeatingArea>, next| match prev {
            Some(p) if p == next => Err(p),
            _ => Ok(Some(next)),
        })
        .unwrap_err();

    stable
        .seats
        .values()
        .filter(|&seat| *seat == Seat::Occupied)
        .count()
}

fn main() {
    let seats = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.expect("Unable to read line")
                .chars()
                .enumerate()
                .filter(|&(_, ch)| ch != '.')
                .map(|(x, _)| ((x as isize, y as isize), Seat::Empty))
                .collect::<Vec<(Point, Seat)>>()
        })
        .collect::<HashMap<Point, Seat>>();

    let area = SeatingArea { seats };

    println!(
        "Part 1: {}",
        solve(&area, |prev| Some(prev.step_immedate()))
    );
    println!(
        "Part 2: {}",
        solve(&area, |prev| Some(prev.step_line_of_sight()))
    );
}
