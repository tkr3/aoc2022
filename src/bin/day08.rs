fn main() {
    let input = include_str!("../../inputs/day08.txt");

    let height_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for x in 0..height_map.len() {
        for y in 0..height_map[0].len() {
            // visualize between 0 and 9 using unicode filled blocks
            print!(
                "{}",
                match height_map[x][y] {
                    1 | 2 => "░",
                    3 | 4 => "▒",
                    5 | 6 => "▓",
                    7..=9 => "█",
                    _ => " ",
                }
            );
        }
        println!();
    }

    println!("Part 1: {}", part_1(&height_map));
    println!("Part 2: {}", part_2(&height_map));
}

fn part_1(height_map: &Vec<Vec<u32>>) -> i32 {
    let mut visible_map = vec![vec![false; height_map[0].len()]; height_map.len()];

    for x in 0..height_map.len() {
        let mut max = 0;
        for y in 0..height_map[0].len() {
            if y == 0 {
                visible_map[x][y] = true;
                max = height_map[x][y];
            } else if height_map[x][y] > max {
                visible_map[x][y] = true;
                max = height_map[x][y];
            }
        }
    }

    for x in 0..height_map.len() {
        let mut max = 0;
        for y in (0..height_map[0].len()).rev() {
            if y == height_map[0].len() - 1 {
                visible_map[x][y] = true;
                max = height_map[x][y];
            } else if height_map[x][y] > max {
                visible_map[x][y] = true;
                max = height_map[x][y];
            }
        }
    }

    for y in 0..height_map[0].len() {
        let mut max = 0;
        for x in 0..height_map.len() {
            if x == 0 {
                visible_map[x][y] = true;
                max = height_map[x][y];
            } else if height_map[x][y] > max {
                visible_map[x][y] = true;
                max = height_map[x][y];
            }
        }
    }

    for y in 0..height_map[0].len() {
        let mut max = 0;
        for x in (0..height_map.len()).rev() {
            if x == height_map.len() - 1 {
                visible_map[x][y] = true;
                max = height_map[x][y];
            } else if height_map[x][y] > max {
                visible_map[x][y] = true;
                max = height_map[x][y];
            }
        }
    }

    visible_map
        .iter()
        .map(|x| x.iter().map(|&b| b as usize).sum::<usize>())
        .sum::<usize>() as i32
}

fn part_2(height_map: &Vec<Vec<u32>>) -> i32 {
    // Calculate maximum scenic score for all points
    let mut max_score = 0;
    for x in 0..height_map.len() {
        for y in 0..height_map[0].len() {
            let score = calculate_scenic_score(height_map, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score as i32
}

fn calculate_scenic_score(height_map: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    max_view_distance(height_map, (x, y), (0, -1))
        * max_view_distance(height_map, (x, y), (0, 1))
        * max_view_distance(height_map, (x, y), (1, 0))
        * max_view_distance(height_map, (x, y), (-1, 0))
}

fn max_view_distance(height_map: &Vec<Vec<u32>>, start: (usize, usize), delta: (i32, i32)) -> u32 {
    let mut distance = 0;
    let start_height = height_map[start.0][start.1];

    while let Some(&height) = height_map
        .get((start.0 as i32 + delta.0 * (distance + 1) as i32) as usize)
        .and_then(|x| x.get((start.1 as i32 + delta.1 * (distance + 1) as i32) as usize))
    {
        distance += 1;
        if height >= start_height {
            break;
        }
    }

    distance
}
