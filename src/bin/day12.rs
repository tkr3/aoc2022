use std::collections::{BTreeMap, HashMap, VecDeque};

fn main() {
    let input = include_str!("../../inputs/day12.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut start: Point = Point(0, 0);
    let mut goal: Point = Point(0, 0);
    let mut height_map = vec![Vec::new(); input.lines().next().unwrap().len()];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            height_map[x].push(if c == 'S' {
                start = Point(x, y);
                char_to_height(&'a')
            } else if c == 'E' {
                goal = Point(x, y);
                char_to_height(&'z')
            } else {
                char_to_height(&c)
            })
        }
    }

    let neighbors = |point: &Point| -> Vec<Point> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(x, y)| (point.0 as i32 + x, point.1 as i32 + y))
            .filter(|(x, y)| {
                (0..height_map.len() as i32).contains(x)
                    && (0..height_map[0].len() as i32).contains(y)
                    && (height_map[*x as usize][*y as usize] as i32
                        - height_map[point.0][point.1] as i32)
                        <= 1
            })
            .map(|(x, y)| Point(x as usize, y as usize))
            .collect()
    };

    let (data, cost) = dijkstra_shortest_path(neighbors, start, goal).expect("No path found");

    print_path(goal, start, data, input);

    cost
}

fn print_path(goal: Point, start: Point, data: BTreeMap<Point, (i32, Point)>, input: &str) {
    let mut path = HashMap::new();
    let mut current = goal;
    while current != start {
        let (_, next) = data.get(&current).unwrap();
        path.insert(next, current);
        current = *next;
    }
    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate() {
            if Point(x, y) == start {
                print!("S");
            } else if Point(x, y) == goal {
                print!("E");
            } else if let Some(next) = path.get(&Point(x, y)) {
                // Print the direction of the next step
                let (dx, dy) = (next.0 as i32 - x as i32, next.1 as i32 - y as i32);
                print!(
                    "{}",
                    match (dx, dy) {
                        (-1, 0) => "←",
                        (1, 0) => "→",
                        (0, -1) => "↑",
                        (0, 1) => "↓",
                        _ => " ",
                    }
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_2(input: &str) -> i32 {
    let mut start_positions: Vec<Point> = Vec::new();
    let mut goal: Point = Point(0, 0);
    let mut height_map = vec![Vec::new(); input.lines().next().unwrap().len()];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            height_map[x].push(if c == 'S' {
                start_positions.push(Point(x, y));
                char_to_height(&'a')
            } else if c == 'E' {
                goal = Point(x, y);
                char_to_height(&'z')
            } else {
                if c == 'a' {
                    start_positions.push(Point(x, y));
                }
                char_to_height(&c)
            })
        }
    }

    let neighbors = |point: &Point| -> Vec<Point> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(x, y)| (point.0 as i32 + x, point.1 as i32 + y))
            .filter(|(x, y)| {
                (0..height_map.len() as i32).contains(x)
                    && (0..height_map[0].len() as i32).contains(y)
                    && (height_map[*x as usize][*y as usize] as i32
                        - height_map[point.0][point.1] as i32)
                        <= 1
            })
            .map(|(x, y)| Point(x as usize, y as usize))
            .collect()
    };

    start_positions
        .into_iter()
        .map(|start| dijkstra_shortest_path(neighbors, start, goal).map(|(_, cost)| cost))
        .filter_map(Result::ok)
        .min()
        .unwrap()
}

fn char_to_height(c: &char) -> u32 {
    *c as u32 - 'a' as u32
}

fn dijkstra_shortest_path<N>(
    neighbors_of: N,
    start: Point,
    goal: Point,
) -> Result<(BTreeMap<Point, (i32, Point)>, i32), ()>
where
    N: Fn(&Point) -> Vec<Point>,
{
    let mut costs = BTreeMap::new();
    let mut queue = VecDeque::new();
    costs.insert(start.clone(), (0, start.clone()));
    queue.push_front(start.clone());

    while let Some(node) = queue.pop_front() {
        let (cost, _) = *costs.get(&node).unwrap();
        if node == goal {
            return Ok((costs, cost));
        }

        for neighbor in neighbors_of(&node) {
            let new_cost = cost + 1;
            if &new_cost < &costs.get(&neighbor).get_or_insert(&(i32::MAX, node)).0 {
                costs.insert(neighbor, (new_cost, node));
                queue.push_back(neighbor);
            }
        }
    }
    return Err(());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(usize, usize);
