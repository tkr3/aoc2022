use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day06.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    solve(4, input)
}

fn part_2(input: &str) -> usize {
    solve(14, input)
}

fn solve(pattern_length: usize, input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    for end in pattern_length..chars.len() {
        if HashSet::<&char>::from_iter(&chars[end - pattern_length..end]).len() == pattern_length {
            return end;
        }
    }
    unreachable!()
}
