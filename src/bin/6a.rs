use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use advent_of_code_2024::input::get_lines;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

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

struct Lab {
    width: usize,
    height: usize,
    obstacles: HashSet<Position>,
}

impl Lab {
    fn add_obstacle(&mut self, x: usize, y: usize) {
        self.obstacles.insert(Position {
            x: x as i32,
            y: y as i32,
        });
    }

    fn has_obstacle(&self, pos: &Position) -> bool {
        self.obstacles.contains(pos)
    }

    fn is_inside(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }
}

struct Guard {
    position: Position,
    direction: Direction,
    lab: Rc<RefCell<Lab>>,
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
}

impl Iterator for Guard {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let before_walk_position = self.position.clone();
        self.walk();
        match self.lab.borrow_mut().is_inside(&before_walk_position) {
            true => Some(before_walk_position),
            false => None,
        }
    }
}

fn main() {
    let lab: Rc<RefCell<Lab>> = Rc::new(RefCell::new(Lab {
        width: 0,
        height: 0,
        obstacles: HashSet::new(),
    }));
    let mut guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::Up,
        lab: lab.clone(),
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
                    guard.position = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    match *cell as char {
                        '^' => guard.direction = Direction::Up,
                        '>' => guard.direction = Direction::Right,
                        'v' => guard.direction = Direction::Down,
                        '<' => guard.direction = Direction::Left,
                        _ => (),
                    }
                }
                _ => (),
            };
        }
        lab.borrow_mut().height += 1;
    }

    let visit_count: usize = guard.into_iter().collect::<HashSet<Position>>().len();

    println!("Places visited: {}", visit_count);
}
