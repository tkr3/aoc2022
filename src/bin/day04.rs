use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../inputs/day04.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let ranges = line
                .splitn(4, &['-', ','][..])
                .map(str::parse)
                .filter_map(Result::ok)
                .collect::<Vec<i32>>();
            return ranges[0] >= ranges[2] && ranges[1] <= ranges[3]
                || ranges[0] <= ranges[2] && ranges[1] >= ranges[3];
        })
        .count() as i32
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let ranges = line
                .splitn(4, &['-', ','][..])
                .map(str::parse)
                .filter_map(Result::ok)
                .collect::<Vec<i32>>()
                .chunks(2)
                .map(|range| RangeInclusive::new(range[0], range[1]))
                .collect::<Vec<RangeInclusive<i32>>>();
            return ranges[0].contains(&ranges[1].start())
                || ranges[0].contains(&ranges[1].end())
                || ranges[1].contains(&ranges[0].start())
                || ranges[1].contains(&ranges[0].end());
        })
        .count() as i32
}
