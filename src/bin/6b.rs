use std::{
    cell::RefCell,
    collections::{BTreeSet, HashSet},
    hash::Hash,
    rc::Rc,
    time::Instant,
};

use advent_of_code_2024::input::get_lines;
use itertools::iproduct;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Position {
    fn r#move(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        };
    }

    fn moved(&self, dir: &Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Direction {
    fn next_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Lab {
    width: usize,
    height: usize,
    obstacles: HashSet<Position>,
    extra_obstacle: Position,
}

impl Lab {
    fn add_obstacle(&mut self, x: usize, y: usize) {
        self.obstacles.insert(Position {
            x: x as i32,
            y: y as i32,
        });
    }

    fn has_obstacle(&self, pos: &Position) -> bool {
        self.has_obstacle_cached(pos, &self.extra_obstacle)
    }

    fn has_obstacle_cached(&self, pos: &Position, extra_pos: &Position) -> bool {
        self.obstacles.contains(pos) || pos == extra_pos
    }

    fn is_inside(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }
}

#[derive(Clone, Debug)]
struct Guard {
    position: Position,
    direction: Direction,
    lab: Rc<RefCell<Lab>>,

    initial_position: Position,
    initial_direction: Direction,
}

impl Guard {
    fn rotate(&mut self) {
        self.direction = self.direction.next_direction();
    }
    fn walk(&mut self) -> bool {
        if !self
            .lab
            .borrow_mut()
            .has_obstacle(&self.position.moved(&self.direction))
        {
            self.position.r#move(&self.direction);
            true
        } else {
            self.rotate();
            false
        }
    }
    fn reset(&mut self) {
        self.position = self.initial_position.clone();
        self.direction = self.initial_direction.clone();
    }
}

impl Iterator for Guard {
    type Item = (Position, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let before_walk_position = self.position.clone();
        let before_walk_direction = self.direction.clone();
        self.walk();
        match self.lab.borrow_mut().is_inside(&before_walk_position) {
            true => Some((before_walk_position, before_walk_direction)),
            false => None,
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let lab: Rc<RefCell<Lab>> = Rc::new(RefCell::new(Lab {
        width: 0,
        height: 0,
        obstacles: HashSet::new(),
        extra_obstacle: Position { x: 0, y: 0 },
    }));
    let mut guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::Up,
        lab: lab.clone(),
        initial_position: Position { x: 0, y: 0 },
        initial_direction: Direction::Up,
    };

    for (y, line_e) in get_lines(6).enumerate() {
        let line = line_e.expect("No line..!");
        if y == 0 {
            lab.borrow_mut().width = line.len();
        }
        for (x, cell) in line.into_bytes().iter().enumerate() {
            match *cell as char {
                '#' => {
                    lab.borrow_mut().add_obstacle(x, y);
                }
                '^' | '>' | 'v' | '<' => {
                    guard.initial_position = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    match *cell as char {
                        '^' => guard.initial_direction = Direction::Up,
                        '>' => guard.initial_direction = Direction::Right,
                        'v' => guard.initial_direction = Direction::Down,
                        '<' => guard.initial_direction = Direction::Left,
                        _ => (),
                    }
                }
                _ => (),
            };
        }
        lab.borrow_mut().height += 1;
    }

    guard.reset();
    let base_visits = guard
        .by_ref()
        .map(|(pos, _dir)| pos)
        .collect::<HashSet<Position>>();

    let mut loops = 0;
    let mut overlaps = 0;
    let mut not_visited = 0;

    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let width = lab.borrow().width.clone();
    let height = lab.borrow().height.clone();
    'p: for (x, y) in iproduct!(0..width, 0..height) {
        visited.clear();
        let extra_obstacle = Position {
            x: x as i32,
            y: y as i32,
        };
        if lab.borrow().has_obstacle(&extra_obstacle) {
            overlaps += 1;
            continue;
        }
        if !base_visits.contains(&extra_obstacle) {
            not_visited += 1;
            continue;
        }
        lab.borrow_mut().extra_obstacle = extra_obstacle;
        guard.reset();
        for visit in guard.by_ref() {
            if visited.contains(&visit) {
                loops += 1;
                println!("LOOP {x}, {y}");
                continue 'p;
            }
            visited.insert(visit);
        }
    }

    let elapsed = start_time.elapsed();

    println!("Loops: {loops}, overlaps {overlaps}, not_visited {not_visited}, time: {elapsed:.2?}");
}
