use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day03.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line[..line.len() / 2]
                .chars()
                .fold(HashSet::new(), |mut set, c| {
                    if line[line.len() / 2..].contains(c) {
                        set.insert(c);
                    }
                    set
                })
                .into_iter()
                .map(|c| c as i32 - 38 - 58 * c.is_lowercase() as i32)
                .sum::<i32>()
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().clone().collect();
    (0..lines.len())
        .step_by(3)
        .map(|i| {
            let mut set: HashSet<char> = HashSet::from_iter(lines[i].chars());
            for line in &lines[i + 1..i + 3] {
                set = set.intersection(&line.chars().collect()).cloned().collect();
            }
            set.into_iter()
                .map(|c| c as i32 - 38 - 58 * c.is_lowercase() as i32)
                .sum::<i32>()
        })
        .sum()
}
