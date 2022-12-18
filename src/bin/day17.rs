use std::{
    collections::{BTreeSet, HashSet},
    thread,
    time::Duration,
    vec,
};

const INPUT: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

fn main() {
    let input = include_str!("../../inputs/day17.txt");

    println!("Part 1: {}", part_1(input));
    // println!("Part 2: {}", part_2(INPUT));
}

fn part_1(input: &str) -> i32 {
    const VISUALIZE: bool = true;
    const WIDTH: usize = 7;
    let mut map: BTreeSet<Point> = BTreeSet::new();
    let mut highest_y: i32 = -1;

    let directions: Vec<char> = input.chars().filter(|&c| c == '>' || c == '<').collect();
    let mut direction_index = 0u32;
    for part in 0..2022 {
        let mut shape = Shape::from_type(2, (highest_y + 4) as usize, part % 5);

        loop {
            let _ = match directions[direction_index as usize] {
                '>' => shape.try_move(1, 0, |x, y| map.contains(&Point::new(x, y))),
                '<' => shape.try_move(-1, 0, |x, y| map.contains(&Point::new(x, y))),
                _ => panic!("Unknown direction"),
            };
            direction_index += 1;
            direction_index %= directions.len() as u32;

            if VISUALIZE {
                for y in (highest_y - 10..highest_y + 10).rev() {
                    print!("{}[2K", 27 as char);
                    print!("|");
                    for x in 0..WIDTH {
                        if shape
                            .blocks
                            .iter()
                            .any(|(bx, by)| shape.x + *bx == x && shape.y + *by == y as usize)
                        {
                            print!("#");
                        } else if map.contains(&Point::new(x, y as usize)) {
                            print!("█");
                        } else if y >= 0 {
                            print!("░");
                        } else {
                            print!(" ");
                        }
                    }
                    if y % 10 == 0 && y > 0 {
                        println!("| --- {} ---", y);
                        continue;
                    }
                    println!("|");
                }
                print!("{}[{}A", 27 as char, 20);
                thread::sleep(Duration::from_millis(300));
            }

            if shape
                .try_move(0, -1, |x, y| map.contains(&Point::new(x, y)))
                .is_err()
            {
                // We hit the ground
                for (x, y) in shape.blocks.iter() {
                    let new_y = shape.y + *y;
                    map.insert(Point::new(shape.x + *x, new_y));
                    highest_y = highest_y.max(new_y as i32);
                }
                break;
            }
        }
    }

    highest_y + 1
}

fn part_2(input: &str) -> u64 {
    let mut map: BTreeSet<Point> = BTreeSet::new();
    let mut highest_y: i32 = -1;

    let mut patterns: HashSet<Vec<u64>> = HashSet::new();

    let mut highest_points: Vec<u64> = vec![0; 7];

    let directions: Vec<char> = input.chars().filter(|&c| c == '>' || c == '<').collect();
    let mut direction_index = 0u32;
    for part in 1.. {
        let mut shape = Shape::from_type(2, (highest_y + 4) as usize, part % 5);

        if patterns.contains(&highest_points) {
            println!("Found pattern at {}", part);
        } else {
            patterns.insert(highest_points.clone());
        }

        loop {
            let _ = match directions[direction_index as usize] {
                '>' => shape.try_move(1, 0, |x, y| map.contains(&Point::new(x, y))),
                '<' => shape.try_move(-1, 0, |x, y| map.contains(&Point::new(x, y))),
                _ => panic!("Unknown direction"),
            };
            direction_index += 1;
            direction_index %= directions.len() as u32;

            if shape
                .try_move(0, -1, |x, y| map.contains(&Point::new(x, y)))
                .is_err()
            {
                // We hit the ground
                for (x, y) in shape.blocks.iter() {
                    let new_y = shape.y + *y;
                    map.insert(Point::new(shape.x + *x, new_y));
                    highest_y = highest_y.max(new_y as i32);

                    if new_y as u64 > highest_points[shape.x + *x] {
                        highest_points[shape.x + *x] = new_y as u64;
                    }
                }
                break;
            }
        }
    }
    (highest_y + 1) as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y))
    }
}

#[derive(Debug, Clone)]
struct Shape {
    x: usize,
    y: usize,
    blocks: Vec<(usize, usize)>,
}

impl Shape {
    fn from_type(x: usize, y: usize, shtype: usize) -> Shape {
        Shape {
            x,
            y,
            blocks: match shtype {
                0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                1 => vec![(1, 0), (0, 1), (2, 1), (1, 2), (1, 1)],
                2 => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
                3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                4 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
                _ => panic!("Unknown shape type: {}", shtype),
            },
        }
    }

    fn try_move<R>(&mut self, dx: i32, dy: i32, is_rock: R) -> Result<(), ()>
    where
        R: Fn(usize, usize) -> bool,
    {
        let (new_x, new_y) = (self.x as i32 + dx, self.y as i32 + dy);

        if (new_x < 0) || (new_y < 0) {
            return Err(());
        }

        for (x, y) in self.blocks.iter() {
            if new_x as usize + *x > 6 || is_rock(new_x as usize + *x, new_y as usize + *y) {
                return Err(());
            }
        }
        self.x = new_x as usize;
        self.y = new_y as usize;
        Ok(())
    }
}
