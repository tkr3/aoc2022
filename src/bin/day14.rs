use std::{collections::HashMap, fmt::Display};
fn main() {
    let input = include_str!("../../inputs/day14.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut map = Map::new();
    for line in input.lines() {
        let points: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|p| p.split_once(",").unwrap())
            .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()))
            .collect();
        for window in points.windows(2) {
            let (start, end) = (window[0], window[1]);
            // Add all points in the line
            for x in start.0.min(end.0)..=start.0.max(end.0) {
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    map.set_material(x, y, Material::Rock);
                }
            }
        }
    }
    // generate sand
    let mut sand_count = 0;
    while let Ok(_) = map.place_sand() {
        sand_count += 1;
    }
    sand_count
}

fn part_2(input: &str) -> i32 {
    let mut map = Map::with_floor();
    for line in input.lines() {
        let points: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|p| p.split_once(",").unwrap())
            .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()))
            .collect();
        for window in points.windows(2) {
            let (start, end) = (window[0], window[1]);
            // Add all points in the line
            for x in start.0.min(end.0)..=start.0.max(end.0) {
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    map.set_material(x, y, Material::Rock);
                }
            }
        }
    }
    // generate sand
    let mut sand_count = 0;
    while let Ok(_) = map.place_sand() {
        sand_count += 1;
    }
    sand_count
}

struct Map {
    map: HashMap<(i32, i32), Material>,
    bounds: (i32, i32, i32, i32),
    floor: bool,
}

impl Map {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            bounds: (i32::MAX, i32::MIN, -1, i32::MIN),
            floor: false,
        }
    }

    fn with_floor() -> Self {
        Self {
            map: HashMap::new(),
            bounds: (i32::MAX, i32::MIN, -1, i32::MIN),
            floor: true,
        }
    }

    fn set_material(&mut self, x: i32, y: i32, material: Material) {
        // update bounds
        self.bounds.0 = self.bounds.0.min(x);
        self.bounds.1 = self.bounds.1.max(x);
        if material == Material::Rock {
            self.bounds.2 = self.bounds.2.min(y);
            self.bounds.3 = self.bounds.3.max(y);
        }

        self.map.insert((x, y), material);
    }

    fn get_material(&self, pos: (i32, i32)) -> Option<&Material> {
        if self.floor && pos.1 >= self.bounds.3 + 2 {
            return Some(&Material::Rock);
        }
        self.map.get(&pos)
    }

    fn place_sand(&mut self) -> Result<(i32, i32), ()> {
        const START_POSITION: (i32, i32) = (500, 0);
        let mut current_pos = START_POSITION;

        if self.get_material(START_POSITION).is_some() {
            return Err(());
        }

        'o: loop {
            if !self.floor && (self.bounds.3 < current_pos.1) {
                return Err(());
            }
            for pos in
                [(0, 1), (-1, 1), (1, 1)].map(|(dx, dy)| (current_pos.0 + dx, current_pos.1 + dy))
            {
                if self.get_material(pos).is_none() {
                    current_pos = pos;
                    continue 'o;
                }
            }
            break;
        }
        self.set_material(current_pos.0, current_pos.1, Material::Sand);
        Ok(current_pos)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds.2..=self.bounds.3 {
            for x in self.bounds.0..=self.bounds.1 {
                let material = self.map.get(&(x, y));
                write!(
                    f,
                    "{}",
                    match material {
                        Some(Material::Rock) => '▓',
                        Some(Material::Sand) => 'o',
                        None => '░',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Material {
    Rock,
    Sand,
}
