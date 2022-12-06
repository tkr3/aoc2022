use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let mut counts = BTreeSet::new();
    input.lines().fold(0, |acc, line: &str| {
        return if line.is_empty() {
            counts.insert(acc);
            0
        } else {
            acc + line.parse::<i32>().unwrap()
        };
    });
    println!("Max calories: {}", counts.iter().last().unwrap());

    println!(
        "Three most calories: {}",
        counts.iter().rev().take(3).sum::<i32>()
    );
}
