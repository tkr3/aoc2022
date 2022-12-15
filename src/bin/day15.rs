use std::{collections::HashSet, error::Error};

fn main() {
    let input = include_str!("../../inputs/day15.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {:?}", part_2(input).unwrap());
}

fn part_1(input: &str) -> usize {
    const Y_COORD: usize = 2000000;
    let (scanners, beacons) = parse_input(input).expect("Failed to parse input");

    let mut scanned_points = HashSet::new();
    for scanner in &scanners {
        let dy = scanner.position.y.abs_diff(Y_COORD as i32) as i32;
        if dy <= scanner.range as i32 {
            for x in scanner.position.x - (scanner.range as i32 - dy)
                ..=scanner.position.x + (scanner.range as i32 - dy)
            {
                let point = Point::new(x, Y_COORD as i32);
                if scanner.is_in_range(&point) {
                    scanned_points.insert(point);
                }
            }
        }
    }
    for beacon in beacons.iter().filter(|b| b.y == Y_COORD as i32) {
        scanned_points.remove(beacon);
    }
    scanned_points.len()
}

fn part_2(input: &str) -> Option<u64> {
    const COORD_MIN: i32 = 0;
    const COORD_MAX: i32 = 4000000;

    let (scanners, _beacons) = parse_input(input).expect("Failed to parse input");
    let mut point = None;
    'o: for y in COORD_MIN..=COORD_MAX {
        let mut x = COORD_MIN;
        while x <= COORD_MAX {
            let pos = Point::new(x, y);
            let scanner = scanners.iter().find(|s| s.is_in_range(&pos));
            match scanner {
                Some(scanner) => {
                    let next_point = scanner.next_unscanned_point(&pos);
                    x = next_point.x;
                    continue;
                }
                None => {
                    point = Some(pos);
                    break 'o;
                }
            }
        }
    }
    if let Some(Point { x, y }) = point {
        Some(x as u64 * 4000000 + y as u64)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Result<(Vec<Scanner>, Vec<Point>), Box<dyn Error>> {
    let mut scanners = Vec::new();
    let mut beacons = HashSet::new();
    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let sensor_position = Point::new(
            line[2]
                .strip_prefix("x=")
                .unwrap()
                .strip_suffix(",")
                .unwrap()
                .parse::<i32>()?,
            line[3]
                .strip_prefix("y=")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse::<i32>()?,
        );
        let beacon_position = Point::new(
            line[8]
                .strip_prefix("x=")
                .unwrap()
                .strip_suffix(",")
                .unwrap()
                .parse::<i32>()?,
            line[9].strip_prefix("y=").unwrap().parse::<i32>()?,
        );
        beacons.insert(beacon_position);

        let scanner_range = sensor_position.distance_to(&beacon_position);
        scanners.push(Scanner {
            position: sensor_position,
            range: scanner_range,
        });
    }
    Ok((scanners, Vec::from_iter(beacons.into_iter())))
}

#[derive(Debug)]
struct Scanner {
    position: Point,
    range: u32,
}

impl Scanner {
    fn is_in_range(&self, point: &Point) -> bool {
        self.position.distance_to(point) <= self.range as u32
    }

    fn next_unscanned_point(&self, pos: &Point) -> Point {
        Point::new(
            self.position.x + (self.range - (pos.y.abs_diff(self.position.y))) as i32 + 1,
            pos.y,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance_to(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
