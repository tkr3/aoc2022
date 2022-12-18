use pathfinding::prelude::bfs;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day18.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut cubes = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        let z = parts.next().unwrap().parse::<i32>().unwrap();

        cubes.push(Point3D::new(x, y, z));
    }
    let mut touching = 0;
    for (index, c1) in cubes.iter().enumerate() {
        for c2 in &cubes[index + 1..] {
            if c1 == c2 {
                continue;
            }
            // Check if c1 and c2 are touching
            let x_diff = c1.x.abs_diff(c2.x);
            let y_diff = c1.y.abs_diff(c2.y);
            let z_diff = c1.z.abs_diff(c2.z);

            if x_diff + y_diff + z_diff == 1 {
                touching += 2;
            }
        }
    }
    cubes.len() as i32 * 6 - touching
}

fn part_2(input: &str) -> usize {
    let mut cubes = Vec::new();
    let mut max_point = Point3D::new(0, 0, 0);
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        let z = parts.next().unwrap().parse::<i32>().unwrap();

        max_point.x = max_point.x.max(x + 1);
        max_point.x = max_point.x.max(x + 1);
        max_point.x = max_point.x.max(x + 1);

        cubes.push(Point3D::new(x, y, z));
    }
    let mut neighbors = HashSet::new();

    for c in &cubes {
        neighbors.extend(c.neighbors().into_iter().filter(|&p| !cubes.contains(&p)));
    }

    let mut air = HashSet::new();
    air.insert(max_point);

    neighbors.retain(|n| {
        if let Some(air_points) = bfs(
            n,
            |&p| p.neighbors().into_iter().filter(|&p| !cubes.contains(&p)),
            |&p| air.contains(&p),
        ) {
            air_points[1..].iter().for_each(|p| {
                air.insert(*p);
            });
            return true;
        }
        false
    });

    let mut sides = HashSet::new();
    for n in neighbors {
        for c in n.neighbors() {
            if cubes.contains(&c) {
                if sides.contains(&(c, n)) {
                    continue;
                }
                sides.insert((n, c));
            }
        }
    }
    sides.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        neighbors.push(Self::new(self.x - 1, self.y, self.z));
        neighbors.push(Self::new(self.x + 1, self.y, self.z));
        neighbors.push(Self::new(self.x, self.y - 1, self.z));
        neighbors.push(Self::new(self.x, self.y + 1, self.z));
        neighbors.push(Self::new(self.x, self.y, self.z - 1));
        neighbors.push(Self::new(self.x, self.y, self.z + 1));
        neighbors
    }
}
