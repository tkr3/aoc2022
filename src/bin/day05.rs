fn main() {
    let input = include_str!("../../inputs/day05.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let parts = lines
        .splitn(2, |&line| line.is_empty())
        .collect::<Vec<&[&str]>>();
    let (initial, instructions) = (parts[0], parts[1]);
    let mut stacks = setup_stacks(initial);

    instructions.into_iter().for_each(|&line| {
        let line = line
            .split_whitespace()
            .map(str::parse)
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();
        let (count, from, to) = (line[0], line[1], line[2]);
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    });
    stacks.into_iter().fold(String::new(), |mut acc, s| {
        acc.push(*s.last().unwrap_or(&' '));
        acc
    })
}

fn part_2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let parts = lines
        .splitn(2, |&line| line.is_empty())
        .collect::<Vec<&[&str]>>();
    let (initial, instructions) = (parts[0], parts[1]);
    let mut stacks = setup_stacks(initial);

    instructions.into_iter().for_each(|&line| {
        let line = line
            .split_whitespace()
            .map(str::parse)
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();
        let (count, from, to) = (line[0], line[1], line[2]);
        let mut temp_stack = Vec::with_capacity(count);
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            temp_stack.push(c);
        }
        for _ in 0..count {
            stacks[to - 1].push(temp_stack.pop().unwrap());
        }
    });
    stacks.into_iter().fold(String::new(), |mut acc, s| {
        acc.push(*s.last().unwrap_or(&' '));
        acc
    })
}

fn setup_stacks(initial: &[&str]) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::new(); 9];
    for &line in initial {
        for (i, c) in line[1..=line.len() - 1].chars().step_by(4).enumerate() {
            if c.is_ascii_uppercase() {
                stacks[i].push(c);
            }
        }
    }
    for s in stacks.iter_mut() {
        s.reverse();
    }
    stacks
}
