use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day06.txt");

    println!("Part 1: {}", part_1(input).unwrap());
    println!("Part 2: {}", part_2(input).unwrap());
}

fn part_1(input: &str) -> Result<usize, ()> {
    let chars = input.chars().collect::<Vec<_>>();
    for end in 4..chars.len() {
        if HashSet::<&char>::from_iter(&chars[end - 4..end]).len() == 4 {
            return Ok(end);
        }
    }
    Err(())
}

fn part_2(input: &str) -> Result<usize, ()> {
    let length = 14;
    let chars = input.chars().collect::<Vec<_>>();
    for end in length..chars.len() {
        if HashSet::<&char>::from_iter(&chars[end - length..end]).len() == length {
            return Ok(end);
        }
    }
    Err(())
}
