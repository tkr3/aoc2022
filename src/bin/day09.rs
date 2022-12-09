use std::{collections::HashSet, ops::AddAssign, str::FromStr};

fn main() {
    let input = include_str!("../../inputs/day09.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut unique_positions = HashSet::new();
    let mut head_position = Point::new(0, 0);
    let mut tail_position = Point::new(0, 0);
    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let direction: Direction = split[0].parse().unwrap();
        let count: usize = split[1].parse().unwrap();
        for _ in 0..count {
            head_position += direction.get_direction();
            move_tail(&head_position, &mut tail_position);
            unique_positions.insert(tail_position);
        }
    }
    unique_positions.len()
}

fn part_2(input: &str) -> usize {
    let mut unique_positions = HashSet::new();
    let mut knots = vec![Point::new(0, 0); 10];
    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let direction: Direction = split[0].parse().unwrap();
        let count: usize = split[1].parse().unwrap();
        for _ in 0..count {
            // Move head
            knots[0] += direction.get_direction();
            // Move each knot
            for i in 1..knots.len() {
                move_tail(&knots[i - 1].clone(), &mut knots[i]);
            }
            // Add tail to set
            unique_positions.insert(knots.last().unwrap().clone());
        }
    }
    unique_positions.len()
}

fn move_tail(head: &Point, tail: &mut Point) {
    if head.x > tail.x + 1 {
        tail.x += 1;
        if head.y > tail.y {
            tail.y += 1;
        } else if head.y < tail.y {
            tail.y -= 1;
        }
    } else if head.x < tail.x - 1 {
        tail.x -= 1;
        if head.y > tail.y {
            tail.y += 1;
        } else if head.y < tail.y {
            tail.y -= 1;
        }
    }
    if head.y > tail.y + 1 {
        tail.y += 1;
        if head.x > tail.x {
            tail.x += 1;
        } else if head.x < tail.x {
            tail.x -= 1;
        }
    } else if head.y < tail.y - 1 {
        tail.y -= 1;
        if head.x > tail.x {
            tail.x += 1;
        } else if head.x < tail.x {
            tail.x -= 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn get_direction(&self) -> Point {
        match self {
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
        }
    }
}
