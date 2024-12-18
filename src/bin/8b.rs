use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

use advent_of_code_2024::input::get_lines;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn within_bounds(&self, min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> bool {
        self.x >= min_x && self.y >= min_y && self.x < max_x && self.y < max_y
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for &Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Extrapolator<'a, 'b> {
    start: &'a Position,
    other: &'b Position,
    pos: usize,
}

impl Extrapolator<'_, '_> {
    fn new<'a, 'b>(start: &'a Position, other: &'b Position) -> Extrapolator<'a, 'b> {
        Extrapolator {
            start,
            other,
            pos: 0,
        }
    }
}

impl Iterator for Extrapolator<'_, '_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > 1 && self.other == &(Position { x: 0, y: 0 }) {
            None
        } else {
            self.pos += 1;
            Some(self.start + &(&(self.other - self.start) * (self.pos as isize)))
        }
    }
}

fn main() {
    let field_entries = get_lines(8)
        .enumerate()
        .map(|(y, line_ex)| {
            line_ex
                .expect("no line..!")
                .chars()
                .enumerate()
                .filter(|(_x, ch)| ch != &'.')
                .map(|(x, ch)| (x, y, ch))
                .collect_vec()
        })
        .flatten()
        .map(|(x, y, ch)| {
            (
                ch,
                Position {
                    x: x as isize,
                    y: y as isize,
                },
            )
        })
        .collect::<Vec<(char, Position)>>();

    let field_width = get_lines(8)
        .next()
        .expect("no first line..!")
        .expect("first line error..!")
        .len();
    let field_height = get_lines(8).count();

    let mut field: HashMap<char, Vec<Position>> = HashMap::new();
    field_entries.iter().for_each(|(ch, xy)| {
        field
            .entry(*ch)
            .and_modify(|e| e.push(*xy))
            .or_insert(vec![*xy]);
    });

    let count_antinodes = field
        .iter()
        .map(|(_ch, xys)| {
            xys.iter()
                .permutations(2)
                .map(|pair| {
                    let pos_filter = |pos: &Position| {
                        pos.within_bounds(0, 0, field_width as isize, field_height as isize)
                    };
                    [
                        Extrapolator::new(pair[0], pair[1]).take_while(pos_filter),
                        Extrapolator::new(pair[1], pair[0]).take_while(pos_filter),
                    ]
                })
                .flatten()
        })
        .flatten()
        .flatten()
        .collect::<HashSet<Position>>()
        .len();

    println!("Number of anti nodes: {count_antinodes}")
}
